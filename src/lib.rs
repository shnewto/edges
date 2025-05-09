#![doc = include_str!("../README.md")]

use binary_image::{BinaryImage, BinaryView, Bit};
use image::{DynamicImage, GenericImageView};

#[cfg(feature = "bevy")]
pub(crate) use bevy_math::prelude::{UVec2, Vec2};
#[cfg(all(not(feature = "bevy"), feature = "glam-latest"))]
pub(crate) use glam::{UVec2, Vec2};

pub extern crate binary_image;
pub use iter::Edges as EdgesIter;

pub mod anchor;
pub mod utils;

#[cfg(feature = "bevy")]
mod bevy;
mod iter;
#[cfg(all(feature = "bevy", test))]
mod tests;

/// A struct representing the edges of a image.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edges<I: GenericImageView<Pixel = Bit>>(pub I);

impl<I> Edges<I>
where
    I: GenericImageView<Pixel = Bit>,
{
    /// Translates the edges of a single image into a coordinate system centered at (0, 0).
    ///
    /// # Returns
    ///
    /// A vector of `Vec2` representing the translated edge points.
    #[inline]
    #[must_use]
    pub fn single_translated(&self) -> Option<Vec<Vec2>> {
        self.iter()
            .next()
            .map(|polygon| self.anchor().translate(polygon))
    }

    /// Retrieves the raw edge points of a single image.
    ///
    /// # Returns
    ///
    /// A vector of `UVec2` representing the raw edge points.
    #[inline]
    #[must_use]
    pub fn single_raw(&self) -> Option<Vec<UVec2>> {
        self.iter().next()
    }

    /// Translates the edges of multiple images into a coordinate system centered at (0, 0).
    ///
    /// # Returns
    ///
    /// A vector of vectors of `Vec2` representing the translated edge points of each image.
    #[inline]
    #[must_use]
    pub fn multi_translated(&self) -> Vec<Vec<Vec2>> {
        self.anchor().translate_polygons(self.iter())
    }

    /// Retrieves the raw edge points of multiple images.
    ///
    /// # Returns
    ///
    /// A vector of vectors of `UVec2` representing the raw edge points of each image.
    #[inline]
    #[must_use]
    pub fn multi_raw(&self) -> Vec<Vec<UVec2>> {
        self.iter().collect()
    }

    #[inline]
    #[must_use]
    pub fn iter(&self) -> iter::Edges<I> {
        self.into_iter()
    }

    #[inline]
    #[must_use]
    fn anchor(&self) -> anchor::Anchor {
        anchor::Anchor::Center(self.height(), self.width())
    }
}

impl From<DynamicImage> for Edges<BinaryImage> {
    fn from(image: DynamicImage) -> Edges<BinaryImage> {
        Self(BinaryImage::from(image))
    }
}

impl<'a> From<&'a DynamicImage> for Edges<BinaryView<'a, DynamicImage>> {
    fn from(image: &'a DynamicImage) -> Edges<BinaryView<'a, DynamicImage>> {
        Self(BinaryView::Ref(image))
    }
}

impl<I> std::fmt::Debug for Edges<I>
where
    I: GenericImageView<Pixel = Bit>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Edges")
            .field("raw", &self.multi_raw())
            .field("translated", &self.multi_translated())
            .finish()
    }
}

impl<I> std::ops::Deref for Edges<I>
where
    I: GenericImageView<Pixel = Bit>,
{
    type Target = I;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, I> IntoIterator for &'a Edges<I>
where
    I: GenericImageView<Pixel = Bit>,
{
    type Item = Vec<UVec2>;
    type IntoIter = iter::Edges<'a, I>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(&self.0)
    }
}
