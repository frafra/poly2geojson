# Description

This is a fast converter from POLY files (Osmosis Polygon Filter File Format) to GeoJSON.

# How to install the latest release

Go to https://github.com/frafra/poly2geojson/releases to download a compiled Linux binary for x86_64 or use `cargo` to build and install it:

```
cargo install poly2geojson
```

# How to build from Git

Cargo should be installed.

```
cargo build --release
```

# How to run

```
poly2geojson < test.poly > test.geojson
```
