use crate::Edges;

use bevy_image::prelude::Image as BevyImage;
use binary_image::BinaryImage;

impl From<BevyImage> for Edges<BinaryImage> {
    fn from(image: BevyImage) -> Edges<BinaryImage> {
        Self::new(BinaryImage::from_raw(
            image.height(),
            image.width(),
            &image.data,
        ))
    }
}

impl TryFrom<&BevyImage> for Edges<BinaryImage> {
    type Error = binary_image::bevy::IntoBinaryImageError;
    fn try_from(image: &BevyImage) -> Result<Edges<BinaryImage>, Self::Error> {
        Ok(Edges::new(BinaryImage::try_from(image)?))
    }
}
