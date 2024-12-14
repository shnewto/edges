#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{utils::center, UVec2, Vec2};

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
    pub fn center(self) -> Option<UVec2> {
        match self {
            Anchor::Center(height, width) => Some(UVec2::new(width, height) / 2),
            Anchor::VerticalCenter(height) => Some(UVec2::new(0, height / 2)),
            Anchor::HorisontalCenter(width) => Some(UVec2::new(width / 2, 0)),
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
        let delta = self
            .center()
            .or_else(|| center(polygon.iter().copied()))
            .unwrap_or(UVec2::ZERO)
            .as_vec2();

        #[cfg(feature = "parallel")]
        let iter = polygon.into_par_iter();
        #[cfg(not(feature = "parallel"))]
        let iter = polygon.into_iter();

        iter.map(|p| Vec2::new(p.x as f32 - delta.x, delta.y - p.y as f32))
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
