#![doc = include_str!("../README.md")]

#[cfg(feature = "bevy")]
pub use bevy_math::prelude::Vec2;
#[cfg(not(feature = "bevy"))]
pub use glam::Vec2;
use std::{collections::HashMap, fmt};

use crate::{
    bin_image::BinImage,
    utils::{distance, Point},
};

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
    pub fn new(height: usize, width: usize, data: &[u8]) -> Self {
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
        let edge_points = (0..image.height * image.width)
            .map(|i| (i / image.height, i % image.height))
            .filter(|p| image.get(*p))
            .filter(|p| (0..8).contains(&image.get_neighbors(*p).iter().filter(|i| **i).count()))
            .collect();

        self.points_to_drawing_order(edge_points, translate)
    }

    /// Takes a collection of coordinates and attempts to sort them according to drawing order
    ///
    /// Pixel sorted so that the distance to previous and next is 1. When there is no pixel left
    /// with distance 1, another group is created and sorted the same way.
    fn points_to_drawing_order(&self, points: Vec<Point>, translate: bool) -> Vec<Vec<Vec2>> {
        if points.is_empty() {
            return Vec::new();
        }

        let mut groups: Vec<Vec<Point>> = Vec::new();
        let mut group: Vec<Point> = Vec::new();
        let mut drawn_points_with_counts = HashMap::new();

        let mut start = points[0];
        let mut current = start;
        group.push(current);
        drawn_points_with_counts.insert(current, 2);

        while drawn_points_with_counts.len() < points.len() {
            if let Some(p) = points
                .iter()
                .filter(|p| (distance(current, p) - 1.0).abs() <= f32::EPSILON)
                .min_by_key(|n| drawn_points_with_counts.get(n).map_or(0, |c| *c))
            {
                current = *p;
                group.push(current);
                if let Some(c) = drawn_points_with_counts.get_mut(p) {
                    *c += 1;
                } else {
                    drawn_points_with_counts.insert(current, 2);
                }
            }

            // we've traversed and backtracked and we're back at the start without reaching the end of the points
            // so we need to start a collecting the points of a new unconnected object
            if current == start {
                // remove the connecting coordinate
                let _ = group.pop();
                groups.push(group.clone());
                group.clear();
                for val in drawn_points_with_counts.values_mut() {
                    *val = 1;
                }

                if let Some(new_start) = points
                    .iter()
                    .find(|p| !drawn_points_with_counts.contains_key(p))
                {
                    start = *new_start;
                    current = start;
                    group.push(current);
                    drawn_points_with_counts.insert(current, 2);
                } else {
                    break;
                }
            }
        }
        groups.push(group);

        let groups = groups
            .into_iter()
            .map(|v| v.into_iter().map(|(x, y)| Vec2::new(x as f32, y as f32)));

        if translate {
            groups.map(|p| self.image.translate(p)).collect()
        } else {
            groups.map(Iterator::collect).collect()
        }
    }
}

#[cfg(feature = "bevy")]
impl From<bevy_render::prelude::Image> for Edges {
    fn from(i: bevy_render::prelude::Image) -> Edges {
        Self::new(i.height() as usize, i.width() as usize, &i.data)
    }
}

impl From<image::DynamicImage> for Edges {
    fn from(i: image::DynamicImage) -> Edges {
        Self::new(i.height() as usize, i.width() as usize, i.as_bytes())
    }
}

impl<T> From<&T> for Edges
where
    T: Clone,
    Edges: From<T>,
{
    fn from(value: &T) -> Self {
        Self::from(value.clone())
    }
}

impl fmt::Debug for Edges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EdgesDisplay {{
    raw: {:#?},
    translated: {:#?}
}}",
            self.image_edges(false),
            self.image_edges(true),
        )
    }
}
