#![doc = include_str!("../README.md")]

use crate::bin_image::BinImage;
#[cfg(feature = "bevy")]
pub use bevy_math::prelude::{UVec2, Vec2};
#[cfg(all(not(feature = "bevy"), feature = "glam-latest"))]
pub use glam::{UVec2, Vec2};
use rayon::prelude::*;
use std::fmt;
use utils::{handle_neighbors, in_polygon};

mod bin_image;
#[cfg(feature = "bevy")]
#[cfg(test)]
mod tests;
mod utils;

/// A struct representing the edges of a image.
#[derive(Clone)]
pub struct Edges {
    image: BinImage,
}

impl Edges {
    /// Creates a new `Edges` instance from the given dimensions and pixel data.
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
        let corners: Vec<UVec2> = self.collect_corners();
        let mut objects: Vec<Vec<UVec2>> = Vec::new();
        if corners.is_empty() {
            return objects;
        }
        while let Some(start) = corners.iter().find(|point| {
            objects
                .par_iter()
                .all(|object| !(object.contains(point) || in_polygon(**point, object)))
        }) {
            objects.push(self.collect_object(*start));
        }
        objects
    }

    fn collect_corners(&self) -> Vec<UVec2> {
        let image = &self.image;
        (0..image.height() * image.width())
            .into_par_iter()
            .map(|i| UVec2::new(i / image.height(), i % image.height()))
            .filter(|p| image.is_corner(*p))
            .collect()
    }

    fn collect_object(&self, start: UVec2) -> Vec<UVec2> {
        let mut object_edges: Vec<UVec2> = vec![start];
        let mut current = start;
        loop {
            let (last, neighbors) = (
                *object_edges.last().unwrap(),
                self.image.get_neighbors(current),
            );
            if last != current {
                object_edges.push(current);
            }
            handle_neighbors(neighbors, last.x.cmp(&current.x), last.y.cmp(&current.y))
                .move_point(&mut current);
            if current == start {
                break object_edges;
            }
        }
    }

    /// Translates an `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
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
impl From<bevy_image::prelude::Image> for Edges {
    fn from(i: bevy_image::prelude::Image) -> Edges {
        Self::new(i.height(), i.width(), &i.data)
    }
}

impl From<image::DynamicImage> for Edges {
    fn from(i: image::DynamicImage) -> Edges {
        Self::new(i.height(), i.width(), i.as_bytes())
    }
}

#[cfg(feature = "bevy")]
impl From<&bevy_image::prelude::Image> for Edges {
    fn from(i: &bevy_image::prelude::Image) -> Edges {
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
        f.debug_struct("Edges")
            .field("raw", &self.image_edges())
            .field("translated", &self.translate_objects(self.image_edges()))
            .finish()
    }
}
