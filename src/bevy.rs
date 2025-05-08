use crate::Edges;

use bevy_image::prelude::Image as BevyImage;
use binary_image::BinaryImage;

impl TryFrom<BevyImage> for Edges<BinaryImage> {
    type Error = binary_image::bevy::IntoBinaryImageError;
    fn try_from(image: BevyImage) -> Result<Edges<BinaryImage>, Self::Error> {
        BinaryImage::try_from(image).map(Self)
    }
}

impl TryFrom<&BevyImage> for Edges<BinaryImage> {
    type Error = binary_image::bevy::IntoBinaryImageError;
    fn try_from(image: &BevyImage) -> Result<Edges<BinaryImage>, Self::Error> {
        BinaryImage::try_from(image).map(Self)
    }
}
