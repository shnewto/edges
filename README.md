# edges

[![Crates.io](https://img.shields.io/crates/v/edges.svg)](https://crates.io/crates/edges)
[![Crates.io](https://img.shields.io/crates/d/edges.svg)](https://crates.io/crates/edges)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/shnewto/edges#license)

get the edges of objects in images with transparency.

## supported image types

- `image::DynamicImage`.
- `bevy::render::texture::Image` (or if you rather, `bevy::prelude::Image`).
- your image if you implement `Into<Edges>` with `Edges::new`.

## using

```rust
use edges::Edges;
use std::path::Path;

let image = image::open(Path::new("assets/car.png"));
let edges = Edges::from(image.unwrap());
println!("{:#?}", edges.single_image_edge_translated());
```

## How it works

This crate is inspired by [a Coding Train video](https://youtu.be/0ZONMNUKTfU)
on implementing "marching squares." It uses a "march through all the values"
approach to identify edges, specifically targeting pixels that have at least
one neighboring pixel that is "empty".

### Key Steps

1. **Identifying Empty Pixels**:

   - The crate determines whether a pixel is "empty" by performing
     a bitwise OR operation on all the bytes for that pixel.
   - In any images, a pixel is considered "empty" if it has a zero value.

2. **Collecting Edge Pixels**:

   - Instead of drawing contours directly,
     the crate keeps track of the actual pixel coordinates that form the edges.
   - It collects all edge pixels that are adjacent
     to each other (i.e., have a distance of 1).

3. **Grouping Pixels**:

   - If there are pixels that are not adjacent
     (i.e., have a distance greater than 1 from any existing group),
     and are not in any group's polygon,
     those pixels initiate a new group.
   - This ensures that all edge pixels are organized into
     distinct objects based on their connectivity.

4. **Output**:
   - The resulting edge coordinates are then available
     for further processing or visualization.

This method allows for efficient edge detection while maintaining
the integrity of the pixel data, making it suitable for images with transparency.

## license

all code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.
