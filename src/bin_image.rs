use crate::{utils::Point, Vec2};

pub struct BinImage {
    data: Vec<u8>,
    pub height: usize,
    pub width: usize,
}

impl BinImage {
    pub fn new(height: usize, width: usize, data: &[u8]) -> Self {
        let compress_step = data.len() / (height * width);
        Self {
            data: data
                .chunks(8 * compress_step)
                .map(|chunk| {
                    chunk
                        .chunks(compress_step)
                        .map(|chunk| chunk.iter().any(|i| *i != 0))
                        .enumerate()
                        .map(|(index, bit)| u8::from(bit) << index)
                        .sum()
                })
                .collect(),
            height,
            width,
        }
    }

    /// get pixel value at given coordinate
    pub fn get(&self, (x, y): Point) -> bool {
        let index = y * self.width + x;
        if let Some(mut byte) = self.data.get(index / 8 /* index of byte */).copied() {
            byte >>= index % 8; // index of bit
            x <= self.width && byte & 1 > 0
        } else {
            false
        }
    }

    pub fn get_neighbors(&self, (x, y): Point) -> [bool; 8] {
        [
            y < usize::MAX && self.get((x, y + 1)),
            y > usize::MIN && self.get((x, y - 1)),
            x < usize::MAX && self.get((x + 1, y)),
            x > usize::MIN && self.get((x - 1, y)),
            x < usize::MAX && y < usize::MAX && self.get((x + 1, y + 1)),
            x > usize::MIN && y > usize::MIN && self.get((x - 1, y - 1)),
            x < usize::MAX && y > usize::MIN && self.get((x + 1, y - 1)),
            x > usize::MIN && y < usize::MAX && self.get((x - 1, y + 1)),
        ]
    }

    /// translate point in positive x,y to either side of (0,0)
    fn translate_point(&self, p: Vec2) -> Vec2 {
        Vec2::new(
            p.x - (self.width as f32 / 2.0 - 1.0),
            (self.height as f32 / 2.0 - 1.0) - p.y,
        )
    }

    /// Translate iterator of points in positive x,y to either side of (0,0)
    #[must_use]
    pub fn translate<T>(&self, v: T) -> Vec<Vec2>
    where
        T: Iterator<Item = Vec2>,
    {
        v.map(|p| self.translate_point(p)).collect()
    }
}
