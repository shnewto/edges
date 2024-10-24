use crate::{UVec2, Vec2};

pub struct BinImage {
    data: Vec<u8>,
    pub height: u32,
    pub width: u32,
}

impl BinImage {
    pub fn new(height: u32, width: u32, data: &[u8]) -> Self {
        assert!(
            data.len() >= (height * width) as usize,
            "data must not be smaller than image dimensions"
        );
        let compress_step = data.len() / (height * width) as usize;
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

    pub fn get(&self, p: UVec2) -> bool {
        let (x, y) = (p.x, p.y);
        let index = y * self.width + x;
        if let Some(mut byte) = self
            .data
            .get((index / 8) as usize) // index of byte
            .copied()
        {
            byte >>= index % 8; // index of bit
            x <= self.width && byte & 1 > 0
        } else {
            false
        }
    }

    pub fn get_neighbors(&self, p: UVec2) -> [bool; 8] {
        let (x, y) = (p.x, p.y);
        [
            y < u32::MAX && self.get((x, y + 1).into()), // North
            y > u32::MIN && self.get((x, y - 1).into()), // South
            x < u32::MAX && self.get((x + 1, y).into()), // East
            x > u32::MIN && self.get((x - 1, y).into()), // West
            x < u32::MAX && y < u32::MAX && self.get((x + 1, y + 1).into()), // Northeast
            x > u32::MIN && y > u32::MIN && self.get((x - 1, y - 1).into()), // Southwest
            x < u32::MAX && y > u32::MIN && self.get((x + 1, y - 1).into()), // Southeast
            x > u32::MIN && y < u32::MAX && self.get((x - 1, y + 1).into()), // Northwest
        ]
    }

    /// translate point in positive x,y to either side of (0,0)
    fn translate_point(&self, p: Vec2) -> Vec2 {
        Vec2::new(
            p.x - (self.width as f32 / 2.0 - 1.0),
            (self.height as f32 / 2.0 - 1.0) - p.y,
        )
    }

    pub fn translate<T>(&self, v: T) -> Vec<Vec2>
    where
        T: Iterator<Item = Vec2>,
    {
        v.map(|p| self.translate_point(p)).collect()
    }
}
