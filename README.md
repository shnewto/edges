# edges

[![Crates.io](https://img.shields.io/crates/v/edges.svg)](https://crates.io/crates/edges)
[![Crates.io](https://img.shields.io/crates/d/edges.svg)](https://crates.io/crates/edges)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/shnewto/edges#license)

Get the edges of objects in images with transparency.

## Supported image types

- `image::DynamicImage`
- `bevy::image::Image` (or if you rather, `bevy::prelude::Image`)

## Using

```rust
use edges::Edges;
use std::path::Path;

let image = image::open(Path::new("assets/car.png"));
let edges = Edges::from(image.unwrap());
println!("{:#?}", edges.single_translated());
```

## How it works

I was inspired by [a coding train (or, coding in the cabana rather)
on an implementation of "marching squares"](https://youtu.be/0ZONMNUKTfU).
So this crate takes a "march through all the values" approach to find edges, i.e.
pixels with at least 1 empty neighboring pixel, but
instead of drawing a contour in place,
it just keeps track of all the actual pixel coordinates. To determine "empty" I bitwise
or all the bytes for each pixel and,
in images with transparency, "empty" is a zero value for the pixel.

After that, we need to put the coordinates in some kind of
"drawing order" so whatever we pass all the points to,
knows how we want the object constructed. For this, the
crate collects all pixels, in order, that are a distance of 1 from each other.
If there are pixels that have a distance greater than 1
from any pixel in an existing group, that pixel begins a new group.

## License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

At your option.
