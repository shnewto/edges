//! Binary (opaque / transparent) image types used by edge detection.

use std::ops::Deref;

use image::{DynamicImage, GenericImageView};

/// A single opaque (`true`) or transparent (`false`) pixel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bit(pub bool);

impl Deref for Bit {
    type Target = bool;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<bool> for Bit {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<Bit> for bool {
    #[inline]
    fn from(value: Bit) -> Self {
        value.0
    }
}

/// Images that can be sampled as opaque / transparent bits.
pub trait BinaryImageView {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    #[inline]
    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    #[inline]
    fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width() && y < self.height()
    }

    fn get_pixel(&self, x: u32, y: u32) -> Bit;
}

/// An owned bit-packed binary image.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BinaryImage {
    width: u32,
    height: u32,
    /// Row-major bits, one `bool` per pixel (`true` = opaque).
    buffer: Vec<bool>,
}

impl BinaryImage {
    #[inline]
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![false; (width * height) as usize],
        }
    }

    /// Build from raw channel bytes. Each pixel is `bytes_per_pixel` long; a pixel is opaque
    /// when any of its bytes is non-zero.
    #[must_use]
    pub fn from_raw(width: u32, height: u32, buffer: &[u8]) -> Self {
        let image_size = (width * height) as usize;
        debug_assert!(
            buffer.len() >= image_size,
            "Buffer must not be smaller than image dimensions"
        );
        let compress_step = buffer.len() / image_size;
        Self {
            width,
            height,
            buffer: buffer
                .chunks(compress_step)
                .map(|pixel| pixel.iter().any(|&b| b != 0))
                .collect(),
        }
    }

    #[inline]
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: Bit) {
        debug_assert!(self.in_bounds(x, y));
        self.buffer[(y * self.width + x) as usize] = pixel.0;
    }

    /// Crop a sub-rectangle out of this image.
    #[must_use]
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let width = width.min(self.width.saturating_sub(x));
        let height = height.min(self.height.saturating_sub(y));
        let mut out = Self::new(width, height);
        for row in 0..height {
            for col in 0..width {
                out.put_pixel(col, row, self.get_pixel(x + col, y + row));
            }
        }
        out
    }

    /// Nearest-neighbor resize.
    #[must_use]
    pub fn resize(&self, width: u32, height: u32) -> Self {
        if width == 0 || height == 0 || self.width == 0 || self.height == 0 {
            return Self::new(width, height);
        }
        let mut out = Self::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let src_x = (x * self.width / width).min(self.width - 1);
                let src_y = (y * self.height / height).min(self.height - 1);
                out.put_pixel(x, y, self.get_pixel(src_x, src_y));
            }
        }
        out
    }

    /// Flip an image horizontally.
    #[must_use]
    pub fn flip_horizontal(&self) -> Self {
        let mut out = Self::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                out.put_pixel(self.width - x - 1, y, self.get_pixel(x, y));
            }
        }
        out
    }

    /// Flip an image vertically.
    #[must_use]
    pub fn flip_vertical(&self) -> Self {
        let mut out = Self::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                out.put_pixel(x, self.height - y - 1, self.get_pixel(x, y));
            }
        }
        out
    }
}

impl BinaryImageView for BinaryImage {
    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Bit {
        debug_assert!(self.in_bounds(x, y));
        Bit(self.buffer[(y * self.width + x) as usize])
    }
}

impl From<DynamicImage> for BinaryImage {
    fn from(image: DynamicImage) -> Self {
        Self::from(&image)
    }
}

impl From<&DynamicImage> for BinaryImage {
    fn from(image: &DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        Self {
            width,
            height,
            buffer: image
                .pixels()
                .map(|(_, _, pixel)| pixel_is_opaque(image, pixel))
                .collect(),
        }
    }
}

/// A borrowed view that interprets another image as binary on read.
#[derive(Debug, Clone, Copy)]
pub enum BinaryView<'a, I> {
    Ref(&'a I),
    Image(I),
}

impl<I> Deref for BinaryView<'_, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(image) => image,
            Self::Image(image) => image,
        }
    }
}

impl BinaryImageView for BinaryView<'_, DynamicImage> {
    #[inline]
    fn width(&self) -> u32 {
        (**self).width()
    }

    #[inline]
    fn height(&self) -> u32 {
        (**self).height()
    }

    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Bit {
        let image = &**self;
        debug_assert!(image.in_bounds(x, y));
        Bit(pixel_is_opaque(image, image.get_pixel(x, y)))
    }
}

impl From<BinaryView<'_, DynamicImage>> for BinaryImage {
    fn from(view: BinaryView<'_, DynamicImage>) -> Self {
        Self::from(&*view)
    }
}

/// Whether a dynamic-image pixel counts as opaque for edge detection.
///
/// Uses alpha when the source has an alpha channel; otherwise any non-zero channel.
fn pixel_is_opaque(image: &DynamicImage, rgba: image::Rgba<u8>) -> bool {
    match image {
        DynamicImage::ImageRgba8(_)
        | DynamicImage::ImageRgba16(_)
        | DynamicImage::ImageRgba32F(_)
        | DynamicImage::ImageLumaA8(_)
        | DynamicImage::ImageLumaA16(_) => rgba[3] != 0,
        _ => rgba.0.iter().take(3).any(|&c| c != 0),
    }
}
