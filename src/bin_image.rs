use std::fmt::Display;

use crate::{utils::is_corner, UVec2, Vec2};
use rayon::prelude::*;
pub mod neighbors {
    /// Neighbor constants for 8-connectivity pixel access.
    pub const NORTH: u8 = 0b1000_0000;
    pub const SOUTH: u8 = 0b0100_0000;
    pub const EAST: u8 = 0b0010_0000;
    pub const WEST: u8 = 0b0001_0000;
    pub const NORTHEAST: u8 = 0b0000_1000;
    pub const NORTHWEST: u8 = 0b0000_0100;
    pub const SOUTHEAST: u8 = 0b0000_0010;
    pub const SOUTHWEST: u8 = 0b0000_0001;
}

/// A struct representing a binary image.
#[derive(Clone, Debug, Default)]
pub struct BinImage {
    data: Vec<u8>,
    height: u32,
    width: u32,
}

impl BinImage {
    /// Creates a new `BinImage` from the given height, width, and raw pixel data.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the image in pixels.
    /// * `width` - The width of the image in pixels.
    /// * `data` - A slice of bytes representing the raw pixel data. The length of this slice
    ///   must be at least `height * width`.
    ///
    /// # Panics
    ///
    /// This function will panic if the length of `data` is less than `height * width`.
    pub fn new(height: u32, width: u32, data: &[u8]) -> Self {
        assert!(
            data.len() >= (height * width) as usize,
            "data must not be smaller than image dimensions"
        );
        let compress_step = data.len() / (height * width) as usize;
        Self {
            data: data
                .par_chunks(8 * compress_step)
                .map(|chunk| {
                    chunk
                        .par_chunks(compress_step)
                        .map(|chunk| chunk.iter().any(|i| *i != 0))
                        .enumerate()
                        .map(|(index, bit)| u8::from(bit) << index)
                        .sum()
                })
                .collect(),
            height,
            width,
        }
    }

    /// Gets the pixel value at the given coordinate.
    ///
    /// # Arguments
    ///
    /// * `p` - A `UVec2` representing the coordinates of the pixel.
    ///
    /// # Returns
    ///
    /// Returns `true` if the pixel is "on" (1), and `false` if it is "off" (0) or out of bounds.
    pub fn get(&self, p: UVec2) -> bool {
        if p.x >= self.width {
            return false;
        }
        let index = p.y * self.width + p.x;
        if let Some(mut byte) = self
            .data
            .get((index / 8) as usize) // index of byte
            .copied()
        {
            byte >>= index % 8; // index of bit
            byte & 1 > 0
        } else {
            false
        }
    }

    /// Gets the values of the neighboring pixels (8-connectivity) around the given coordinate.
    ///
    /// # Arguments
    ///
    /// * `p` - A `UVec2` representing the coordinates of the center pixel.
    ///
    /// # Returns
    ///
    /// An byte representing the state of the neighboring pixels.
    pub fn get_neighbors(&self, p: UVec2) -> u8 {
        let (x, y) = (p.x, p.y);
        let mut neighbors = 0;
        if y < u32::MAX && self.get(UVec2::new(x, y + 1)) {
            neighbors |= neighbors::NORTH;
        }
        if y > u32::MIN && self.get(UVec2::new(x, y - 1)) {
            neighbors |= neighbors::SOUTH;
        }
        if x < u32::MAX && self.get(UVec2::new(x + 1, y)) {
            neighbors |= neighbors::EAST;
        }
        if x > u32::MIN && self.get(UVec2::new(x - 1, y)) {
            neighbors |= neighbors::WEST;
        }
        if x < u32::MAX && y < u32::MAX && self.get(UVec2::new(x + 1, y + 1)) {
            neighbors |= neighbors::NORTHEAST;
        }
        if x > u32::MIN && y < u32::MAX && self.get(UVec2::new(x - 1, y + 1)) {
            neighbors |= neighbors::NORTHWEST;
        }
        if x < u32::MAX && y > u32::MIN && self.get(UVec2::new(x + 1, y - 1)) {
            neighbors |= neighbors::SOUTHEAST;
        }
        if x > u32::MIN && y > u32::MIN && self.get(UVec2::new(x - 1, y - 1)) {
            neighbors |= neighbors::SOUTHWEST;
        }
        neighbors
    }

    pub fn is_corner(&self, p: UVec2) -> bool {
        self.get(p) && is_corner(self.get_neighbors(p))
    }

    /// Translates a point in positive (x, y) coordinates to a coordinate system centered at (0, 0).
    ///
    /// # Arguments
    ///
    /// * `p` - A `Vec2` representing the point to translate.
    ///
    /// # Returns
    ///
    /// A new `Vec2` representing the translated coordinates
    const fn translate_point(&self, p: UVec2) -> Vec2 {
        Vec2::new(
            p.x as f32 - (self.width / 2) as f32,
            (self.height / 2) as f32 - p.y as f32,
        )
    }

    /// Translates an `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
    ///
    /// # Arguments
    ///
    /// * `v` - An `Vec` of `Vec2` points to translate.
    ///
    /// # Returns
    ///
    /// A vector of `Vec2` representing the translated coordinates.
    pub fn translate(&self, v: Vec<UVec2>) -> Vec<Vec2> {
        v.into_par_iter().map(|p| self.translate_point(p)).collect()
    }

    /// Gets the height of the image.
    ///
    /// # Returns
    ///
    /// The height of the image in pixels.
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Gets the width of the image.
    ///
    /// # Returns
    ///
    /// The width of the image in pixels.
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Crops the image to a specified rectangular area defined by the `min` and `max` coordinates.
    /// Adjusting automatically if the coordinates exceed the image boundaries.
    ///
    /// # Parameters
    /// - `min`: The top-left corner of the rectangle to crop. This should be a `UVec2` representing the x and y coordinates.
    /// - `max`: The bottom-right corner of the rectangle to crop. This should also be a `UVec2` representing the x and y coordinates.
    ///
    /// # Returns
    /// A new `Self` (the cropped image) containing only the pixels within the specified rectangle. If the crop area is invalid (e.g., zero width or height), an empty image will be returned.
    ///
    /// # Panics
    /// This function will panic if the `min` coordinates are greater than the `max` coordinates.
    pub fn crop(&self, min: UVec2, max: UVec2) -> Self {
        assert!(min.x <= max.x && min.y <= max.y, "Invalid crop coordinates");
        let max = max.min(UVec2::new(self.width(), self.height()));
        let min = min.min(UVec2::new(self.width(), self.height()));
        let (height, width) = (max.y - min.y, max.x - min.x);
        if height * width == 0 {
            return Self::default();
        }
        let mut data: Vec<u8> = vec![0; ((width + 7) / 8 * height) as usize];
        (0..(height * width))
            .filter(|&i| self.get(UVec2::new(min.x + i % width, min.y + i / width)))
            .for_each(|i| {
                let byte_index = (i / 8) as usize;
                let bit_index = i % 8;
                data[byte_index] |= 1 << bit_index;
            });
        Self {
            data,
            height,
            width,
        }
    }
}

impl Display for BinImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get(UVec2::new(x, y)) {
                    write!(f, "█")?;
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
