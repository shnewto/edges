#[cfg(feature = "bevy")]
pub use bevy_math::prelude::Vec2;
#[cfg(not(feature = "bevy"))]
pub use glam::Vec2;
use std::fmt;

use hashbrown::HashSet;
use mashmap::MashMap;
use ordered_float::OrderedFloat;

pub struct Edges {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

impl Edges {
    #[must_use]
    pub fn new(height: usize, width: usize, data: &[u8]) -> Self {
        let new = Self {
            height,
            width,
            data: {
                let binary = if data.len() > (height * width) {
                    let mut binary = Vec::with_capacity(height * width);
                    let compress_step = data.len() / (height * width);
                    for i in (0..data.len()).step_by(compress_step) {
                        binary.push(data[i..i + compress_step].iter().any(|i| *i > 0));
                    }
                    binary
                } else {
                    data.iter().map(|i| *i > 0).collect()
                };
                let mut compressed = Vec::with_capacity(height * width / 8);
                for i in (0..binary.len()).step_by(8) {
                    let mut byte = 0;
                    for bit in &binary[i..i + 7] {
                        byte += u8::from(*bit);
                        byte <<= 1;
                    }
                    compressed.push(byte);
                }
                compressed
            },
        };
        new
    }
    /// If there's only one sprite / object in the image, this returns just one, with
    /// coordinates translated to either side of (0, 0)
    #[must_use]
    pub fn single_image_edge_translated(&self) -> Vec<Vec2> {
        self.image_edges(true).into_iter().flatten().collect()
    }

    /// If there's only one sprite / object in the image, this returns just one, with
    /// coordinates left alone and all in positive x and y
    #[must_use]
    pub fn single_image_edge_raw(&self) -> Vec<Vec2> {
        self.image_edges(false).into_iter().flatten().collect()
    }

    /// If there's more than one sprite / object in the image, this returns all it finds, with
    /// coordinates translated to either side of (0, 0)
    #[must_use]
    pub fn multi_image_edge_translated(&self) -> Vec<Vec<Vec2>> {
        self.image_edges(true)
    }

    /// If there's more than one sprite / object in the image, this returns all it finds, with
    /// coordinates left alone and all in positive x and y
    #[must_use]
    pub fn multi_image_edges_raw(&self) -> Vec<Vec<Vec2>> {
        self.image_edges(false)
    }

    /// Takes `Edges` and a boolean to indicate whether to translate
    /// the points you get back to either side of (0, 0) instead of everything in positive x and y.
    #[must_use]
    pub fn image_edges(&self, translate: bool) -> Vec<Vec<Vec2>> {
        let mut edge_points: Vec<Vec2> = Vec::new();
        // Marching squares adjacent, walks all the pixels in the provided data and keeps track of
        // any that have at least one transparent / zero value neighbor then, while sorting into drawing
        // order, groups them into sets of connected pixels
        for i in 0..self.width * self.height {
            let (x, y) = self.get_pos(i);
            if !self.get(x, y) {
                continue;
            }
            let neighbors = [
                y < usize::MAX && self.get(x, y + 1),
                y > usize::MIN && self.get(x, y - 1),
                x < usize::MAX && self.get(x + 1, y),
                x > usize::MIN && self.get(x - 1, y),
                x < usize::MAX && y < usize::MAX && self.get(x + 1, y + 1),
                x > usize::MIN && y > usize::MIN && self.get(x - 1, y - 1),
                x < usize::MAX && y > usize::MIN && self.get(x + 1, y - 1),
                x > usize::MIN && y < usize::MAX && self.get(x - 1, y + 1),
            ];
            if neighbors.iter().filter(|i| **i).count() < neighbors.len() {
                edge_points.push(Vec2::new(x as f32, y as f32));
            }
        }

        self.points_to_drawing_order(edge_points, translate)
    }

    /// Takes a collection of coordinates and attempts to sort them according to drawing order
    ///
    /// Pixel sorted so that the distance to previous and next is 1. When there is no pixel left
    /// with distance 1, another group is created and sorted the same way.
    fn points_to_drawing_order(&self, points: Vec<Vec2>, translate: bool) -> Vec<Vec<Vec2>> {
        let mut groups: Vec<Vec<Vec2>> = Vec::new();
        if points.is_empty() {
            return groups;
        }

        let mut in_drawing_order: Vec<Vec2> = Vec::new();
        let mut drawn_points_with_counts: MashMap<(OrderedFloat<f32>, OrderedFloat<f32>), ()> =
            MashMap::new();
        let mut drawn_points: HashSet<(OrderedFloat<f32>, OrderedFloat<f32>)> = HashSet::new();
        let hashable = |v: Vec2| (OrderedFloat(v.x), OrderedFloat(v.y));
        // d=√((x2-x1)²+(y2-y1)²)
        let distance = |a: Vec2, b: Vec2| -> f32 {
            ((a.x - b.x).abs().powi(2) + (a.y - b.y).abs().powi(2)).sqrt()
        };

        let mut start = points[0];
        let mut current = start;
        in_drawing_order.push(current);
        drawn_points_with_counts.insert(hashable(current), ());
        drawn_points.insert(hashable(current));

        while drawn_points.len() < points.len() {
            let neighbors = &points
                .iter()
                .filter(|p| (distance(current, **p) - 1.0).abs() < 0.000_000_1)
                .collect::<Vec<&Vec2>>();

            if let Some(p) = neighbors
                .iter()
                .min_by_key(|n| drawn_points_with_counts.get_iter(&hashable(***n)).count())
            {
                current = **p;
                in_drawing_order.push(**p);
                drawn_points_with_counts.insert(hashable(**p), ());
                drawn_points.insert(hashable(**p));
            }

            // we've traversed and backtracked and we're back at the start without reaching the end of the points
            // so we need to start a collecting the points of a new unconnected object
            if current == start {
                // remove the connecting coordinate
                _ = in_drawing_order.pop();
                groups.push(in_drawing_order.clone());
                in_drawing_order.clear();
                drawn_points_with_counts.clear();

                if let Some(c) = points
                    .iter()
                    .find(|p| !drawn_points.contains(&hashable(**p)))
                {
                    in_drawing_order.push(*c);
                    drawn_points_with_counts.insert(hashable(*c), ());
                    drawn_points.insert(hashable(*c));
                    current = *c;
                    start = current;
                } else {
                    break;
                }
            }
        }

        groups.push(in_drawing_order.clone());

        if translate {
            groups = groups.into_iter().map(|p| self.translate_vec(p)).collect();
        }

        groups
    }

    /// conceptual helper, access a 1D vector like it's a 2D vector
    fn get_pos(&self, index: usize) -> (usize, usize) {
        let quot = index / self.height;
        let rem = index % self.height;
        (quot, rem)
    }

    /// get zero or non-zero pixel the value at given coordinate
    fn get(&self, x: usize, y: usize) -> bool {
        let index = y * self.width / 8 + x / 8;
        if let Some(mut byte) = self.data.get(index).copied() {
            byte >>= 7 - x % 8;
            byte & 1 > 0
        } else {
            false
        }
    }

    /// translate point in positive x,y to either side of (0,0)
    fn translate_point(&self, p: Vec2) -> Vec2 {
        Vec2::new(
            p.x - (self.width as f32 / 2.0 - 1.0),
            (self.height as f32 / 2.0 - 1.0) - p.y,
        )
    }

    /// Translate vector of points in positive x,y to either side of (0,0)
    #[must_use]
    pub fn translate_vec(&self, v: Vec<Vec2>) -> Vec<Vec2> {
        v.into_iter().map(|p| self.translate_point(p)).collect()
    }
}

#[cfg(feature = "bevy")]
impl From<bevy_render::prelude::Image> for Edges {
    fn from(i: bevy_render::prelude::Image) -> Edges {
        Self::new(i.height() as usize, i.width() as usize, &i.data)
    }
}

impl From<image::DynamicImage> for Edges {
    fn from(i: image::DynamicImage) -> Edges {
        Self::new(i.height() as usize, i.width() as usize, i.as_bytes())
    }
}

impl<T> From<&T> for Edges
where
    T: Clone,
    Edges: From<T>,
{
    fn from(value: &T) -> Self {
        Self::from(value.clone())
    }
}

impl fmt::Debug for Edges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct EdgesDisplay {
            raw: Vec<Vec<Vec2>>,
            translated: Vec<Vec<Vec2>>,
        }

        let edges_display = EdgesDisplay {
            raw: self.image_edges(false),
            translated: self.image_edges(true),
        };
        write!(f, "{edges_display:#?}")
    }
}

#[cfg(feature = "bevy")]
#[cfg(test)]
mod tests {
    use crate::Edges;
    use bevy_render::{
        render_asset::RenderAssetUsages,
        texture::{CompressedImageFormats, Image, ImageSampler, ImageType},
    };
    use std::path::Path;

    #[test]
    fn same_image_same_edges() {
        let dynamic_image = image::open(Path::new("assets/car.png")).unwrap();
        let dynamic_edges = Edges::from(dynamic_image);

        let bevy_image = Image::from_buffer(
            include_bytes!("../assets/car.png"), // buffer
            ImageType::Extension("png"),
            CompressedImageFormats::default(),
            true, //
            ImageSampler::default(),
            RenderAssetUsages::default(),
        )
        .unwrap();
        let bevy_edges = Edges::from(bevy_image);

        assert_eq!(
            dynamic_edges.single_image_edge_raw(),
            bevy_edges.single_image_edge_raw()
        );
        assert_eq!(
            dynamic_edges.single_image_edge_translated(),
            bevy_edges.single_image_edge_translated()
        );
    }

    #[test]
    fn same_images_same_edges() {
        let dynamic_image = image::open(Path::new("assets/boulders.png")).unwrap();
        let dynamic_edges = Edges::from(dynamic_image);

        let bevy_image = Image::from_buffer(
            include_bytes!("../assets/boulders.png"), // buffer
            ImageType::Extension("png"),
            CompressedImageFormats::default(),
            true, //
            ImageSampler::default(),
            RenderAssetUsages::default(),
        )
        .unwrap();
        let bevy_edges = Edges::from(bevy_image);

        assert_eq!(
            dynamic_edges.multi_image_edges_raw(),
            bevy_edges.multi_image_edges_raw()
        );
        assert_eq!(
            dynamic_edges.multi_image_edge_translated(),
            bevy_edges.multi_image_edge_translated()
        );
    }
}
