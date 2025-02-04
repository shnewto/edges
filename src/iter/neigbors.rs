use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, Into};
use image::GenericImageView;

use binary_image::Bit;

bitflags::bitflags! {
    /// Neighbor constants for 8-connectivity pixel access.
    #[repr(transparent)]
    #[derive(
        Clone,
        Copy,
        Debug,
        Display,
        Default,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Deref,
        DerefMut,
        AsMut,
        AsRef,
        Into,
        From,
    )]
    pub struct Neighbors: u8  {
        const NORTH     = 1 << 7;
        const SOUTH     = 1 << 6;
        const EAST      = 1 << 5;
        const WEST      = 1 << 4;
        const NORTHEAST = 1 << 3;
        const NORTHWEST = 1 << 2;
        const SOUTHEAST = 1 << 1;
        const SOUTHWEST = 1;
    }
}

impl Neighbors {
    #[must_use]
    pub fn from_image<I>(image: &I, mut x: u32, mut y: u32) -> Self
    where
        I: GenericImageView<Pixel = Bit>,
    {
        let mut neighbors = Neighbors::empty();
        if y < u32::MAX {
            y += 1;
            if image.in_bounds(x, y) && *image.get_pixel(x, y) {
                neighbors |= Neighbors::NORTH;
            }
            if x < u32::MAX {
                x += 1;
                if image.in_bounds(x, y) && *image.get_pixel(x, y) {
                    neighbors |= Neighbors::NORTHEAST;
                }
                x -= 1;
            }
            if x > u32::MIN {
                x -= 1;
                if image.in_bounds(x, y) && *image.get_pixel(x, y) {
                    neighbors |= Neighbors::NORTHWEST;
                }
                x += 1;
            }
            y -= 1;
        }
        if x < u32::MAX {
            x += 1;
            if image.in_bounds(x, y) && *image.get_pixel(x, y) {
                neighbors |= Neighbors::EAST;
            }
            if y > u32::MIN {
                y -= 1;
                if image.in_bounds(x, y) && *image.get_pixel(x, y) {
                    neighbors |= Neighbors::SOUTHEAST;
                }
                y += 1;
            }
            x -= 1;
        }
        if y > u32::MIN {
            y -= 1;
            if image.in_bounds(x, y) && *image.get_pixel(x, y) {
                neighbors |= Neighbors::SOUTH;
            }
            if x > u32::MIN {
                x -= 1;
                if image.in_bounds(x, y) && *image.get_pixel(x, y) {
                    neighbors |= Neighbors::SOUTHWEST;
                }
                x += 1;
            }
            y += 1;
        }
        if x > u32::MIN {
            x -= 1;
            if image.in_bounds(x, y) && *image.get_pixel(x, y) {
                neighbors |= Neighbors::WEST;
            }
        }
        neighbors
    }

    #[inline]
    #[must_use]
    pub fn is_corner(self) -> bool {
        !matches!(
            self.bits(),
            255
                | 239
                | 238
                | 235
                | 234
                | 223
                | 221
                | 215
                | 213
                | 188..=207
                | 127
                | 123
                | 119
                | 115
                | 48..=63
                | 9
                | 6
                | 0
        )
    }
}
