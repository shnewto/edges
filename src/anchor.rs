#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{utils::center_of, UVec2, Vec2};

#[derive(Debug, Clone, Copy, Default)]
pub enum Anchor {
    Center(u32, u32),
    VerticalCenter(u32),
    HorisontalCenter(u32),
    #[default]
    AbsoluteCenter,
}

impl Anchor {
    #[inline]
    #[must_use]
    pub fn size(self) -> Option<UVec2> {
        match self {
            Anchor::Center(height, width) => Some(UVec2::new(width, height)),
            Anchor::VerticalCenter(height) => Some(UVec2::new(0, height)),
            Anchor::HorisontalCenter(width) => Some(UVec2::new(width, 0)),
            Anchor::AbsoluteCenter => None,
        }
    }

    /// Translates an `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
    ///
    /// # Returns
    ///
    /// A vector of `Vec2` representing the translated coordinates.
    #[inline]
    #[must_use]
    pub fn translate(self, polygon: Vec<UVec2>) -> Vec<Vec2> {
        let center = self.size().map_or_else(
            || center_of(polygon.iter().copied()).unwrap_or(Vec2::ZERO),
            |size| size.as_vec2() / 2.,
        );
        #[cfg(feature = "parallel")]
        let iter = polygon.into_par_iter();
        #[cfg(not(feature = "parallel"))]
        let iter = polygon.into_iter();

        iter.map(|p| Vec2::new(p.x as f32 - center.x, center.y - p.y as f32))
            .collect()
    }

    /// Translates an `Vec` of `Vec` of points in positive (x, y) coordinates to a coordinate system centered at (0, 0).
    ///
    /// # Returns
    ///
    /// A vector of vector of `Vec2` representing the translated objects.
    #[inline]
    #[must_use]
    pub fn translate_polygons(self, polygons: impl Iterator<Item = Vec<UVec2>>) -> Vec<Vec<Vec2>> {
        polygons.map(|polygon| self.translate(polygon)).collect()
    }
}
