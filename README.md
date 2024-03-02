# edges

get the edges of objects in images with transparency

## disclaimer

this existed first as implementation over in a crate called [bevy_collider_gen](<https://github.com/shnewto/bevy_collider_gen>). at the time, that's what it seemed most useful for. this crate represents me starting to wonder whether it's useful for something else, but at the moment it the api is a bit of a nuisance to use.

i'm working through how to make it less of a nuisance, and if I succeed, I'll be using it over in the [bevy_collider_gen](<https://github.com/shnewto/bevy_collider_gen>) crate.

## how it works

i was inspired by [a coding train (or, coding in the cabana rather) on an implementation of "marching squares"](<https://youtu.be/0ZONMNUKTfU>).
so this crate takes a "march through all the values" approach to find edges, i.e. pixels with at least 1 empty neighboring pixel, but
instead of drawing a contour in place, it just keeps track of all the actual pixel coordinates. to determine "empty" I bitwise
or all the bytes for each pixel and, in images with transparency, "empty" is a zero value for the pixel.

after that, we need to put the coordinates in some kind of "drawing order" so whatever we pass all the points to, knows how we want the object constructed. for this, the
crate collects all pixels, in order, that are a distance of 1 from eachother. if there are pixels that have a distance greater than 1
from any pixel in an existing group, that pixel begins a new group.

## todo

- examples
- allow input for specifying the pixel value you want to treat as "transparency"

## license

all code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.