#![doc = include_str!("../README.md")]

#[cfg(feature = "bevy")]
pub use bevy_math::prelude::{UVec2, Vec2};
use binary_image::{BinaryImage, BinaryView, Bit, Neighbors};
#[cfg(all(not(feature = "bevy"), feature = "glam-latest"))]
pub use glam::{UVec2, Vec2};
use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;
use std::fmt;
use utils::{bounding_box, handle_neighbors};

#[cfg(feature = "bevy")]
#[cfg(test)]
mod tests;
mod utils;

/// A struct representing the edges of a image.
#[derive(Clone)]
pub struct Edges<I>
where
    I: GenericImageView<Pixel = Bit>,
{
    pub image: I,
}

impl<I> Edges<I>
where
    I: GenericImageView<Pixel = Bit>,
{
    pub fn new(image: I) -> Self {
        Self { image }
    }

    /// Translates the edges of a single image into a coordinate system centered at (0, 0).
    ///
    /// # Returns
    ///
    /// A vector of `Vec2` representing the translated edge points.
    #[inline]
    #[must_use]
    pub fn single_image_edge_translated(&self) -> Option<Vec<Vec2>> {
        self.single_image_edge_raw().map(translate)
    }

    /// Retrieves the raw edge points of a single image.
    ///
    /// # Returns
    ///
    /// A vector of `UVec2` representing the raw edge points.
    #[inline]
    #[must_use]
    pub fn single_image_edge_raw(&self) -> Option<Vec<UVec2>> {
        let mut corners: Vec<UVec2> = self.collect_corners();
        self.collect_object(&mut corners)
    }

    /// Translates the edges of multiple images into a coordinate system centered at (0, 0).
    ///
    /// # Returns
    ///
    /// A vector of vectors of `Vec2` representing the translated edge points of each image.
    #[inline]
    #[must_use]
    pub fn multi_image_edge_translated(&self) -> Vec<Vec<Vec2>> {
        translate_objects(self.multi_image_edge_raw())
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
        let mut corners: Vec<UVec2> = self.collect_corners();
        let mut objects: Vec<Vec<UVec2>> = Vec::new();
        if corners.is_empty() {
            return objects;
        }
        while let Some(object) = self.collect_object(&mut corners) {
            objects.push(object);
        }
        objects
    }

    fn collect_corners(&self) -> Vec<UVec2> {
        (0..self.image.width())
            .flat_map(|x| (0..self.image.height()).map(move |y| (x, y)))
            .filter(|&(x, y)| Neighbors::is_corner(&self.image, x, y))
            .map(|(x, y)| UVec2::new(x, y))
            .rev()
            .collect()
    }

    fn collect_object(&self, corners: &mut Vec<UVec2>) -> Option<Vec<UVec2>> {
        if let Some(start) = corners.pop() {
            let mut current = start;
            let mut last = start;
            let mut object: Vec<UVec2> = vec![start];
            Some(loop {
                let neighbors = Neighbors::get_neighbors(&self.image, current.x, current.y);
                let dir = handle_neighbors(
                    neighbors.bits(),
                    last.x.cmp(&current.x),
                    last.y.cmp(&current.y),
                );
                let Some(next) = dir.find_next(current, corners) else {
                    todo!("\n{dir:?}\n{neighbors:b} {current}")
                };
                last = current;
                current = next;

                if current == start {
                    *corners = corners
                        .par_iter()
                        .copied()
                        .filter(|p| !(object.contains(p) && utils::in_polygon(*p, &object)))
                        .collect();
                    break object;
                }
                object.push(current);
            })
        } else {
            None
        }
    }
}

impl<I> From<Edges<I>> for Vec<Vec<UVec2>>
where
    I: GenericImageView<Pixel = Bit>,
{
    fn from(value: Edges<I>) -> Vec<Vec<UVec2>> {
        value.image_edges()
    }
}

#[cfg(feature = "bevy")]
impl From<bevy_image::prelude::Image> for Edges<BinaryImage> {
    fn from(image: bevy_image::prelude::Image) -> Edges<BinaryImage> {
        Self {
            image: BinaryImage::from_raw(image.height(), image.width(), &image.data),
        }
    }
}

#[cfg(feature = "bevy")]
impl From<&bevy_image::prelude::Image> for Edges<BinaryImage> {
    fn from(image: &bevy_image::prelude::Image) -> Edges<BinaryImage> {
        Self {
            image: BinaryImage::from_raw(image.height(), image.width(), &image.data),
        }
    }
}

impl From<image::DynamicImage> for Edges<BinaryImage> {
    fn from(image: image::DynamicImage) -> Edges<BinaryImage> {
        Self {
            image: BinaryImage::from(&BinaryView(&image)),
        }
    }
}

impl<'a> From<&'a image::DynamicImage> for Edges<BinaryView<'a, DynamicImage>> {
    fn from(image: &'a image::DynamicImage) -> Edges<BinaryView<'a, DynamicImage>> {
        Self {
            image: BinaryView(image),
        }
    }
}

impl<I> fmt::Debug for Edges<I>
where
    I: GenericImageView<Pixel = Bit>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Edges")
            .field("raw", &self.image_edges())
            .field("translated", &translate_objects(self.image_edges()))
            .finish()
    }
}

/// Translates an `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
///
/// # Returns
///
/// A vector of `Vec2` representing the translated coordinates.
#[inline]
#[must_use]
pub fn translate(polygon: Vec<UVec2>) -> Vec<Vec2> {
    let Some((min, max)) = bounding_box(&polygon) else {
        return Vec::new();
    };
    let size = ((max - min) / 2).as_vec2();
    polygon
        .into_par_iter()
        .map(|p| p.as_vec2() - size)
        .collect()
}

/// Translates an `Vec` of `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
///
/// # Returns
///
/// A vector of vector of `Vec2` representing the translated objects.
#[inline]
#[must_use]
pub fn translate_objects(polygons: Vec<Vec<UVec2>>) -> Vec<Vec<Vec2>> {
    polygons.into_par_iter().map(translate).collect()
}
