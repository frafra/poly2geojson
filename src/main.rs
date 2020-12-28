extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::{self, Read};
use std::path::{Path};

use rusqlite::{Connection, Result, LoadExtensionGuard};

use pest::Parser;

#[derive(Parser)]
#[grammar = "poly.pest"]
pub struct PolyParser;

fn load_spatialite(conn: &Connection) -> Result<()> {
    let _guard = LoadExtensionGuard::new(conn)?;

    conn.load_extension(Path::new("mod_spatialite"), None)
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();

    let file = PolyParser::parse(Rule::file, &buffer)
        .expect("unsuccessful parse")
        .next().unwrap();
    
    let conn = Connection::open(":memory:")?;
    load_spatialite(&conn)?;

    let mut complete_wkt = String::new();
    for file_pair in file.into_inner() {
        match file_pair.as_rule() {
            Rule::ring => {
                let mut subtract = false;
                let mut wkt = String::from("POLYGON((");
                for polygon_pair in file_pair.into_inner() {
                    match polygon_pair.as_rule() {
                        Rule::name => (),
                        Rule::subtract => {
                            subtract = true;
                        },
                        Rule::point => {
                            let mut x: f64 = 0.0;
                            let mut y: f64 = 0.0;
                            for point_pair in polygon_pair.into_inner() {
                                match point_pair.as_rule() {
                                    Rule::x => {
                                        x = point_pair
                                            .as_str().parse().unwrap();
                                    },
                                    Rule::y => {
                                        y = point_pair
                                            .as_str().parse().unwrap();
                                    },
                                    _ => unreachable!(),
                                }
                            }
                            wkt.push_str(&x.to_string());
                            wkt.push(' ');
                            wkt.push_str(&y.to_string());
                            wkt.push(',');
                        },
                        _ => unreachable!(),
                    }
                }
                wkt.pop(); // remove last comma
                wkt.push_str("))");
                if complete_wkt.is_empty() {
                    complete_wkt = wkt;
                    continue;
                }
                if subtract {
                    complete_wkt = conn.query_row("
                        SELECT AsText(ST_Difference(
                            SetSRID(GeomFromText(?1), 4326),
                            SetSRID(GeomFromText(?2), 4326)
                        ))
                    ", &[complete_wkt, wkt], |row| row.get(0)).unwrap()
                } else {
                    complete_wkt = conn.query_row("
                        SELECT AsText(ST_Union(
                            SetSRID(GeomFromText(?1), 4326),
                            SetSRID(GeomFromText(?2), 4326)
                        ))
                    ", &[complete_wkt, wkt], |row| row.get(0)).unwrap();
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    let geojson: String = conn.query_row("
            SELECT AsGeoJSON(
                SetSRID(GeomFromText(?1), 4326)
            )
        ", &[&complete_wkt], |row| row.get(0)).unwrap();
    println!("{}", geojson);

    Ok(())
}
