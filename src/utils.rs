#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{UVec2, Vec2};

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

    let d = UVec2::new(width, height) / 2;
    iter.map(|p| {
        let (x, y) = (p.x.abs_diff(d.x) as f32, p.y.abs_diff(d.y) as f32);
        let cmp = p.cmplt(d);
        Vec2::new(if cmp.x { -x } else { x }, if cmp.y { y } else { -y })
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
