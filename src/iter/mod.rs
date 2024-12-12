use binary_image::{Bit, Neighbors};
use image::GenericImageView;

use crate::{
    utils::{collect_corners, in_polygon},
    UVec2,
};
use direction::Direction;

mod direction;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Edges<'a, I>
where
    I: GenericImageView<Pixel = Bit>,
{
    image: &'a I,
    corners: Vec<UVec2>,
}

impl<'a, I> Edges<'a, I>
where
    I: GenericImageView<Pixel = Bit>,
{
    pub fn new(image: &'a I) -> Self {
        Self {
            image,
            corners: collect_corners(image),
        }
    }
}

impl<'a, I> Iterator for Edges<'a, I>
where
    I: GenericImageView<Pixel = Bit>,
{
    type Item = Vec<UVec2>;
    fn next(&mut self) -> Option<Self::Item> {
        let corners = &mut self.corners;
        corners.pop().map(|start| {
            let mut current = start;
            let mut object = vec![start];

            let neighbors = Neighbors::get_neighbors(self.image, start.x, start.y);
            let mut previous_direction = Direction::next_direction(None, neighbors);

            loop {
                let neighbors = Neighbors::get_neighbors(self.image, current.x, current.y);
                let direction = Direction::next_direction(Some(previous_direction), neighbors);

                current = if previous_direction.reverse() == direction {
                    *object.last().unwrap()
                } else {
                    direction
                        .find_by_direction(current, corners)
                        .unwrap_or(start)
                };

                if *object.last().unwrap() == start && object.contains(&current) {
                    object.pop();
                    corners.retain(|p| !(object.contains(p) || in_polygon(p.x, p.y, &object)));
                    break object;
                }

                previous_direction = direction;
                object.push(current);
            }
        })
    }
}
