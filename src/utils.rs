use crate::{UVec2, Vec2};

use binary_image::{Bit, Neighbors};
use image::GenericImageView;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

// Get the bounding box of the polygon
pub fn bounding_box(polygon: &[UVec2]) -> Option<(UVec2, UVec2)> {
    polygon
        .iter()
        .copied()
        .zip(polygon.iter().copied())
        .reduce(|(min, max), (a, b)| (min.min(a), max.max(b)))
}

pub fn in_polygon(x: u32, y: u32, polygon: &[UVec2]) -> bool {
    if let Some((min, max)) = bounding_box(polygon) {
        // Check if the is within the bounding box
        if x < min.x || x > max.x || y < min.y || y > max.y {
            return false; // Early exit if outside the bounding box
        }
    }

    let mut is_inside = false;

    for (p1, p2) in polygon.windows(2).map(|win| (win[0], win[1])) {
        let (min, max) = (p1.min(p2), p1.max(p2));
        let (dy, dx) = (max.y - min.y, max.x - min.x);

        if min.y <= y && y < max.y && x <= min.x + dx * (y - min.y) / dy {
            if min.x <= x && x < max.x {
                return true;
            }
            is_inside = !is_inside;
        }
    }
    is_inside
}

pub fn collect_corners<I>(image: &I) -> Vec<UVec2>
where
    I: GenericImageView<Pixel = Bit>,
{
    let (width, height) = image.dimensions();
    (0..height)
        .rev()
        .flat_map(|y| (0..width).map(move |x| UVec2::new(x, y)))
        .filter(|p| Neighbors::is_corner(image, p.x, p.y))
        .collect()
}

/// Translates an `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
///
/// # Returns
///
/// A vector of `Vec2` representing the translated coordinates.
#[inline]
#[must_use]
pub fn translate(polygon: Vec<UVec2>, width: u32, height: u32) -> Vec<Vec2> {
    #[cfg(feature = "parallel")]
    let iter = polygon.into_par_iter();
    #[cfg(not(feature = "parallel"))]
    let iter = polygon.into_iter();
    iter.map(|p| {
        Vec2::new(
            p.x as f32 - (width / 2) as f32,
            (height / 2) as f32 - p.y as f32,
        )
    })
    .collect()
}

/// Translates an `Vec` of `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
///
/// # Returns
///
/// A vector of vector of `Vec2` representing the translated objects.
#[inline]
#[must_use]
pub fn translate_objects(
    polygons: impl Iterator<Item = Vec<UVec2>>,
    width: u32,
    height: u32,
) -> Vec<Vec<Vec2>> {
    polygons
        .map(|polygon| translate(polygon, width, height))
        .collect()
}
