use std::fmt;

use glam::Vec2;
use hashbrown::HashSet;
use mashmap::MashMap;
use ordered_float::OrderedFloat;

pub enum Edges {
    DynamicImage(image::DynamicImage),
    #[cfg(feature = "bevy")]
    BevyImage(bevy::prelude::Image),
}

impl Edges {
    /// If there's only one sprite / object in the image, this returns just one, with
    /// coordinates translated to either side of (0, 0)
    pub fn single_image_edge_translated(&self) -> Vec<Vec2> {
        self.image_edges(true).into_iter().flatten().collect()
    }

    /// If there's only one sprite / object in the image, this returns just one, with
    /// coordinates left alone and all in positive x and y
    pub fn single_image_edge_raw(&self) -> Vec<Vec2> {
        self.image_edges(false).into_iter().flatten().collect()
    }

    /// If there's more than one sprite / object in the image, this returns all it finds, with
    /// coordinates translated to either side of (0, 0)
    pub fn multi_image_edge_translated(&self) -> Vec<Vec<Vec2>> {
        self.image_edges(true)
    }

    /// If there's more than one sprite / object in the image, this returns all it finds, with
    /// coordinates left alone and all in positive x and y
    pub fn multi_image_edges_raw(&self) -> Vec<Vec<Vec2>> {
        self.image_edges(false)
    }

    /// Takes a Bevy DynamicImage type and an boolean to indicate whether to translate
    /// the points you get back to either side of (0, 0) instead of everything in positive x and y
    pub fn image_edges(&self, translate: bool) -> Vec<Vec<Vec2>> {
        let rows = self.height();
        let cols = self.width();
        let data: &[u8] = self.bytes();
        let mut byte_combine_step: usize = 1;
        if (rows * cols) < data.len() {
            byte_combine_step = data.len() / (rows * cols);
        }

        let mut processed: Vec<usize> = vec![];
        for i in (0..data.len()).step_by(byte_combine_step) {
            let mut b: usize = 0;
            for j in 0..byte_combine_step {
                b |= data[i + j] as usize; // just need to retain any non-zero values
            }
            processed.push(b);
        }

        Edges::march_edges(&processed, rows, cols, translate)
    }

    /// Marching squares adjacent, walks all the pixels in the provided data and keeps track of
    /// any that have at least one transparent / zero value neighbor then, while sorting into drawing
    /// order, groups them into sets of connected pixels
    ///
    /// Accepts a flag indicating whether or not to translate coordinates to either side of (0,0)
    /// or leave it all in positive x,y
    pub fn march_edges(
        data: &[usize],
        rows: usize,
        cols: usize,
        translate: bool,
    ) -> Vec<Vec<Vec2>> {
        let mut edge_points: Vec<Vec2> = vec![];

        for d in 0..data.len() {
            let (x, y) = Edges::get_xy(d, rows);
            let (c, r) = (x as isize, y as isize);

            if Edges::get_at(r, c, rows, cols, data) == 0 {
                continue;
            }

            let neighbors = [
                Edges::get_at(r + 1, c, rows, cols, data),
                Edges::get_at(r - 1, c, rows, cols, data),
                Edges::get_at(r, c + 1, rows, cols, data),
                Edges::get_at(r, c - 1, rows, cols, data),
                Edges::get_at(r + 1, c + 1, rows, cols, data),
                Edges::get_at(r - 1, c - 1, rows, cols, data),
                Edges::get_at(r + 1, c - 1, rows, cols, data),
                Edges::get_at(r - 1, c + 1, rows, cols, data),
            ];

            let n: usize = neighbors.iter().sum();
            let surrounded = neighbors.len();
            if n < surrounded {
                edge_points.push(Vec2::new(x, y));
            }
        }

        Edges::points_to_drawing_order(&edge_points, translate, rows, cols)
    }

    /// Takes a collection of coordinates and attempts to sort them according to drawing order
    ///
    /// Pixel sorted so that the distance to previous and next is 1. When there is no pixel left
    /// with distance 1, another group is created and sorted the same way.
    fn points_to_drawing_order(
        points: &[Vec2],
        translate: bool,
        rows: usize,
        cols: usize,
    ) -> Vec<Vec<Vec2>> {
        let mut groups: Vec<Vec<Vec2>> = vec![];
        let mut in_drawing_order: Vec<Vec2> = vec![];
        let mut drawn_points_with_counts: MashMap<(OrderedFloat<f32>, OrderedFloat<f32>), ()> =
            MashMap::new();
        let mut drawn_points: HashSet<(OrderedFloat<f32>, OrderedFloat<f32>)> = HashSet::new();
        let hashable = |v: Vec2| (OrderedFloat(v.x), OrderedFloat(v.y));
        if points.is_empty() {
            return groups;
        }

        let mut current = points[0];
        let mut start = current;
        in_drawing_order.push(current);
        drawn_points_with_counts.insert(hashable(current), ());
        drawn_points.insert(hashable(current));

        while drawn_points.len() < points.len() {
            let neighbors = &points
                .iter()
                .filter(|p| Edges::distance(current, **p) == 1.0)
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
            groups = groups
                .into_iter()
                .map(|p| Edges::translate_vec(p, rows, cols))
                .collect();
        }

        groups
    }

    /// conceptual helper, access a 1D vector like it's a 2D vector
    fn get_xy(idx: usize, offset: usize) -> (f32, f32) {
        let quot = idx / offset;
        let rem = idx % offset;
        (quot as f32, rem as f32)
    }

    /// pythagoras, distance between two points
    fn distance(a: Vec2, b: Vec2) -> f32 {
        // d=√((x2-x1)²+(y2-y1)²)
        ((a.x - b.x).abs().powi(2) + (a.y - b.y).abs().powi(2)).sqrt()
    }

    /// get zero or non-zero pixel the value at given coordinate
    fn get_at(row: isize, col: isize, rows: usize, cols: usize, data: &[usize]) -> usize {
        if row < 0 || col < 0 || row >= rows as isize || col >= cols as isize {
            0
        } else {
            let idx = row as usize * cols + col as usize;
            data.get(idx)
                .map(|i| if *i == 0 { 0 } else { 1 })
                .unwrap_or_else(|| 0)
        }
    }

    /// translate point in positive x,y to either side of (0,0)
    fn xy_translate(p: Vec2, rows: usize, cols: usize) -> Vec2 {
        Vec2::new(
            p.x - (cols as f32 / 2. - 1.0),
            -p.y + (rows as f32 / 2. - 1.0),
        )
    }

    /// Translate vector of points in positive x,y to either side of (0,0)
    pub fn translate_vec(v: Vec<Vec2>, rows: usize, cols: usize) -> Vec<Vec2> {
        v.into_iter()
            .map(|p| Edges::xy_translate(p, rows, cols))
            .collect()
    }

    fn width(&self) -> usize {
        match self {
            Edges::DynamicImage(i) => i.width() as usize,
            #[cfg(feature = "bevy")]
            Edges::BevyImage(i) => i.size().x as usize,
        }
    }

    fn height(&self) -> usize {
        match self {
            Edges::DynamicImage(i) => i.height() as usize,
            #[cfg(feature = "bevy")]
            Edges::BevyImage(i) => i.size().y as usize,
        }
    }

    fn bytes(&self) -> &[u8] {
        match self {
            Edges::DynamicImage(i) => i.as_bytes(),
            #[cfg(feature = "bevy")]
            Edges::BevyImage(i) => &i.data,
        }
    }
}

#[cfg(feature = "bevy")]
impl From<bevy::prelude::Image> for Edges {
    fn from(i: bevy::prelude::Image) -> Edges {
        Edges::BevyImage(i)
    }
}

#[cfg(feature = "bevy")]
impl From<&bevy::prelude::Image> for Edges {
    fn from(i: &bevy::prelude::Image) -> Edges {
        Edges::BevyImage(i.clone())
    }
}

impl From<image::DynamicImage> for Edges {
    fn from(i: image::DynamicImage) -> Edges {
        Edges::DynamicImage(i)
    }
}

impl From<&image::DynamicImage> for Edges {
    fn from(i: &image::DynamicImage) -> Edges {
        Edges::DynamicImage(i.clone())
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
        write!(f, "{:#?}", edges_display)
    }
}

#[cfg(feature = "bevy")]
#[cfg(test)]
mod tests {
    use crate::Edges;
    use bevy::render::texture::{Image, ImageType};
    use std::path::Path;

    #[test]
    fn same_image_same_edges() {
        let dynamic_image = image::open(Path::new("assets/car.png")).unwrap();
        let dynamic_edges = Edges::from(dynamic_image);

        let bevy_image = Image::from_buffer(
            include_bytes!("../assets/car.png"), // buffer
            ImageType::Extension("png"),
            Default::default(),
            true, //
            Default::default(),
            Default::default(),
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
            Default::default(),
            true, //
            Default::default(),
            Default::default(),
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
