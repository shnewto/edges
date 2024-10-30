#![doc = include_str!("../README.md")]

use crate::bin_image::BinImage;
#[cfg(feature = "bevy")]
pub use bevy_math::prelude::{UVec2, Vec2};
#[cfg(not(feature = "bevy"))]
pub use glam::{UVec2, Vec2};
use std::fmt;
use utils::{is_corner, match_neighbors};

mod bin_image;
#[cfg(feature = "bevy")]
#[cfg(test)]
mod tests;
mod utils;

pub struct Edges {
    image: BinImage,
}

impl Edges {
    #[must_use]
    pub fn new(height: u32, width: u32, data: &[u8]) -> Self {
        Self {
            image: BinImage::new(height, width, data),
        }
    }

    /// If there's only one sprite / object in the image, this returns just one, with
    /// coordinates translated to either side of (0, 0)
    #[must_use]
    pub fn single_image_edge_translated(&self) -> Vec<Vec2> {
        self.image_edges(true).into_iter().flatten().collect()
    }

    /// If there's only one sprite / object in the image, this returns just one, with
    /// coordinates left alone and all in positive x and y
    #[must_use]
    pub fn single_image_edge_raw(&self) -> Vec<Vec2> {
        self.image_edges(false).into_iter().flatten().collect()
    }

    /// If there's more than one sprite / object in the image, this returns all it finds, with
    /// coordinates translated to either side of (0, 0)
    #[must_use]
    pub fn multi_image_edge_translated(&self) -> Vec<Vec<Vec2>> {
        self.image_edges(true)
    }

    /// If there's more than one sprite / object in the image, this returns all it finds, with
    /// coordinates left alone and all in positive x and y
    #[must_use]
    pub fn multi_image_edges_raw(&self) -> Vec<Vec<Vec2>> {
        self.image_edges(false)
    }

    /// Takes `Edges` and a boolean to indicate whether to translate
    /// the points you get back to either side of (0, 0) instead of everything in positive x and y.
    #[must_use]
    pub fn image_edges(&self, translate: bool) -> Vec<Vec<Vec2>> {
        let image = &self.image;
        // Marching squares adjacent, walks all the pixels in the provided data and keeps track of
        // any that have at least one transparent / zero value neighbor then, while sorting into drawing
        // order, groups them into sets of connected pixels
        let edge_points: Vec<_> = (0..image.height() * image.width())
            .map(|i| UVec2::new(i / image.height(), i % image.height()))
            .filter(|p| image.get(*p) && is_corner(image.get_neighbors(*p)))
            .collect();

        let groups: Vec<_> = self
            .points_to_drawing_order(&edge_points)
            .into_iter()
            .map(|group| group.into_iter().map(|p| p.as_vec2()).collect())
            .collect();
        if translate {
            groups
                .into_iter()
                .map(|group| self.translate(group))
                .collect()
        } else {
            groups
        }
    }

    /// Takes a collection of coordinates and attempts to sort them according to drawing order
    ///
    /// Pixel sorted so that the distance to previous and next is 1. When there is no pixel left
    /// with distance 1, another group is created and sorted the same way.
    fn points_to_drawing_order(&self, points: &[UVec2]) -> Vec<Vec<UVec2>> {
        if points.is_empty() {
            return Vec::new();
        }

        let mut groups: Vec<Vec<_>> = Vec::new();
        let mut group: Vec<_> = Vec::new();
        let mut start = points[0];
        let mut current = start;
        group.push(current);

        loop {
            match_neighbors(self.image.get_neighbors(current), &mut current, &mut group);

            // we've traversed and backtracked and we're back at the start without reaching the end of the points
            // so we need to start a collecting the points of a new unconnected object
            if current == start {
                groups.push(group.clone());
                group.clear();

                if let Some(new_start) = points
                    .iter()
                    .find(|p1| groups.iter().flatten().all(|p2| *p1 != p2))
                {
                    start = *new_start;
                    current = start;
                    group.push(current);
                } else {
                    break;
                }
            }
        }

        groups
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
    #[must_use]
    pub fn translate(&self, v: Vec<Vec2>) -> Vec<Vec2> {
        self.image.translate(v)
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
            "{}",
            format!(
                "Edges {{\nraw: {:#?},\ntranslated: {:#?}\n}}",
                self.image_edges(false),
                self.image_edges(true),
            )
            .replace('\n', "\n    "),
        )
    }
}
