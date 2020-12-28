extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::{self, Read};

use geojson::{Feature, GeoJson, Geometry, Value};

use pest::Parser;

#[derive(Parser)]
#[grammar = "poly.pest"]
pub struct PolyParser;


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let file = PolyParser::parse(Rule::file, &buffer)
        .expect("unsuccessful parse")
        .next().unwrap();
    
    let mut multipolygon: Vec<Vec<Vec<Vec<f64>>>> = Vec::new();

    for file_pair in file.into_inner() {
        match file_pair.as_rule() {
            Rule::ring => {
                let mut subtract = false;
                let mut points = vec![];
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
                            points.push(vec![x, y]);
                        },
                        _ => unreachable!(),
                    }
                }
                if subtract {
                    multipolygon[0].push(points);
                } else {
                    multipolygon.push(vec![points]);
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    let geometry = Geometry::new(Value::MultiPolygon(multipolygon));
    let geojson = GeoJson::Feature(Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: None,
        foreign_members: None,
    });

    let geojson_string = geojson.to_string();
    println!("{}", geojson_string);

    Ok(())
}
