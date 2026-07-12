use crate::{binary::BinaryImage, Edges};

use bevy_image::prelude::Image as BevyImage;
use derive_more::derive::{Display, Error};

impl TryFrom<BevyImage> for BinaryImage {
    type Error = IntoBinaryImageError;

    fn try_from(image: BevyImage) -> Result<Self, Self::Error> {
        let dynamic = image
            .try_into_dynamic()
            .map_err(IntoBinaryImageError::from)?;
        Ok(Self::from(dynamic))
    }
}

impl TryFrom<&BevyImage> for BinaryImage {
    type Error = IntoBinaryImageError;

    fn try_from(image: &BevyImage) -> Result<Self, Self::Error> {
        Self::try_from(image.clone())
    }
}

impl TryFrom<BevyImage> for Edges<BinaryImage> {
    type Error = IntoBinaryImageError;

    fn try_from(image: BevyImage) -> Result<Edges<BinaryImage>, Self::Error> {
        BinaryImage::try_from(image).map(Self)
    }
}

impl TryFrom<&BevyImage> for Edges<BinaryImage> {
    type Error = IntoBinaryImageError;

    fn try_from(image: &BevyImage) -> Result<Edges<BinaryImage>, Self::Error> {
        BinaryImage::try_from(image).map(Self)
    }
}

/// Error converting a Bevy [`Image`](BevyImage) into a binary image for edge detection.
#[non_exhaustive]
#[derive(Error, Display, Debug)]
pub enum IntoBinaryImageError {
    /// Underlying Bevy → [`image::DynamicImage`] conversion failed.
    #[display("{_0}")]
    DynamicImage(bevy_image::IntoDynamicImageError),
}

impl From<bevy_image::IntoDynamicImageError> for IntoBinaryImageError {
    fn from(value: bevy_image::IntoDynamicImageError) -> Self {
        Self::DynamicImage(value)
    }
}
