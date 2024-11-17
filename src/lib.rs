#![doc = include_str!("../README.md")]

use crate::bin_image::BinImage;
#[cfg(feature = "bevy")]
pub use bevy_math::prelude::{UVec2, Vec2};
#[cfg(all(not(feature = "bevy"), feature = "glam-latest"))]
pub use glam::{UVec2, Vec2};
use rayon::prelude::*;
use std::fmt;
use utils::{handle_neighbors, in_polygon, Direction};

mod bin_image;
#[cfg(feature = "bevy")]
#[cfg(test)]
mod tests;
mod utils;

pub struct Edges {
    image: BinImage,
}

impl Edges {
    /// Creates a new `Edges` instance from the given dimensions and pixel data.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the image.
    /// * `width` - The width of the image.
    /// * `data` - A slice of bytes representing the pixel data of the image.
    ///
    #[inline]
    #[must_use]
    pub fn new(height: u32, width: u32, data: &[u8]) -> Self {
        Self {
            image: BinImage::new(height, width, data),
        }
    }

    /// Translates the edges of a single image into a coordinate system centered at (0, 0).
    ///
    /// # Returns
    ///
    /// A vector of `Vec2` representing the translated edge points.
    #[inline]
    #[must_use]
    pub fn single_image_edge_translated(&self) -> Vec<Vec2> {
        self.translate(self.single_image_edge_raw())
    }

    /// Retrieves the raw edge points of a single image.
    ///
    /// # Returns
    ///
    /// A vector of `UVec2` representing the raw edge points.
    #[inline]
    #[must_use]
    pub fn single_image_edge_raw(&self) -> Vec<UVec2> {
        self.image_edges().into_par_iter().flatten().collect()
    }

    /// Translates the edges of multiple images into a coordinate system centered at (0, 0).
    ///
    /// # Returns
    ///
    /// A vector of vectors of `Vec2` representing the translated edge points of each image.
    #[inline]
    #[must_use]
    pub fn multi_image_edge_translated(&self) -> Vec<Vec<Vec2>> {
        self.translate_objects(self.multi_image_edge_raw())
    }

    /// Retrieves the raw edge points of multiple images.
    ///
    /// # Returns
    ///
    /// A vector of vectors of `UVec2` representing the raw edge points of each image.
    #[inline]
    #[must_use]
    pub fn multi_image_edge_raw(&self) -> Vec<Vec<UVec2>> {
        self.image_edges()
    }

    /// Takes `Edges` and a boolean to indicate whether to translate
    /// the points you get back to either side of (0, 0) instead of everything in positive x and y.
    #[must_use]
    pub fn image_edges(&self) -> Vec<Vec<UVec2>> {
        let image = &self.image;
        // Marching squares adjacent, walks all the pixels in the provided data and keeps track of
        // any that have at least one transparent / zero value neighbor then, while sorting into drawing
        // order, groups them into sets of connected pixels
        let corners: Vec<_> = (0..image.height() * image.width())
            .into_par_iter()
            .map(|i| UVec2::new(i / image.height(), i % image.height()))
            .filter(|p| image.is_corner(*p))
            .collect();

        self.collect_objects(&corners)
    }

    fn collect_objects(&self, corners: &[UVec2]) -> Vec<Vec<UVec2>> {
        if corners.is_empty() {
            return Vec::new();
        }

        let mut objects: Vec<Vec<UVec2>> = Vec::new();

        while let Some(start) = corners.iter().find(|point| {
            objects
                .par_iter()
                .all(|object| !(object.contains(point) || in_polygon(**point, object)))
        }) {
            let mut current = *start;
            let mut group: Vec<UVec2> = Vec::new();
            group.push(current);
            let object = loop {
                let (last, neighbors) = (*group.last().unwrap(), self.image.get_neighbors(current));
                if last != current {
                    group.push(current);
                }
                match handle_neighbors(current, last, neighbors) {
                    Direction::North => current.y += 1,
                    Direction::South => current.y -= 1,
                    Direction::East => current.x += 1,
                    Direction::West => current.x -= 1,
                    Direction::Northeast => {
                        current.x += 1;
                        current.y += 1;
                    }
                    Direction::Northwest => {
                        current.x -= 1;
                        current.y += 1;
                    }
                    Direction::Southeast => {
                        current.x += 1;
                        current.y -= 1;
                    }
                    Direction::Southwest => {
                        current.x -= 1;
                        current.y -= 1;
                    }
                }
                if current == *start {
                    break group;
                }
            };
            objects.push(object);
        }

        objects
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
    #[inline]
    #[must_use]
    pub fn translate(&self, v: Vec<UVec2>) -> Vec<Vec2> {
        self.image.translate(v)
    }

    /// Translates an `Vec` of `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
    ///
    /// # Arguments
    ///
    /// * `v` - An `Vec` of `Vec2` points to translate.
    ///
    /// # Returns
    ///
    /// A vector of vector of `Vec2` representing the translated objects.
    #[inline]
    #[must_use]
    pub fn translate_objects(&self, v: Vec<Vec<UVec2>>) -> Vec<Vec<Vec2>> {
        v.into_par_iter()
            .map(|v| self.translate(v))
            .collect::<Vec<_>>()
    }
}

impl From<Edges> for Vec<Vec<UVec2>> {
    fn from(value: Edges) -> Vec<Vec<UVec2>> {
        value.image_edges()
    }
}

#[cfg(feature = "bevy")]
impl From<bevy_render::prelude::Image> for Edges {
    fn from(i: bevy_render::prelude::Image) -> Edges {
        Self::new(i.height(), i.width(), &i.data)
    }
}

impl From<image::DynamicImage> for Edges {
    fn from(i: image::DynamicImage) -> Edges {
        Self::new(i.height(), i.width(), i.as_bytes())
    }
}

#[cfg(feature = "bevy")]
impl From<&bevy_render::prelude::Image> for Edges {
    fn from(i: &bevy_render::prelude::Image) -> Edges {
        Self::new(i.height(), i.width(), &i.data)
    }
}

impl From<&image::DynamicImage> for Edges {
    fn from(i: &image::DynamicImage) -> Edges {
        Self::new(i.height(), i.width(), i.as_bytes())
    }
}

impl fmt::Debug for Edges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Edges {{{}\n}}",
            format!(
                "\nraw: {:#?},\ntranslated: {:#?}",
                self.image_edges(),
                self.translate_objects(self.image_edges())
            )
            .replace('\n', "\n    "),
        )
    }
}
