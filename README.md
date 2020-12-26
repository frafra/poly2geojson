# Description

This is a fast converter from POLY files (Osmosis Polygon Filter File Format) to GeoJSON.

# How to build

Cargo should be installed.

```
cargo build --release
```

# How to run

```
./target/release/poly2geojson < test.poly > test.geojson
```
