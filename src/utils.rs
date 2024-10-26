use crate::{UVec2, Vec2};
use std::collections::HashMap;

// d=√((x2-x1)²+(y2-y1)²)
pub fn distance(a: Vec2, b: Vec2) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// Takes a collection of coordinates and attempts to sort them according to drawing order
///
/// Pixel sorted so that the distance to previous and next is 1. When there is no pixel left
/// with distance 1, another group is created and sorted the same way.
pub fn points_to_drawing_order(points: Vec<UVec2>) -> Vec<Vec<UVec2>> {
    if points.is_empty() {
        return Vec::new();
    }

    let mut groups: Vec<Vec<UVec2>> = Vec::new();
    let mut group: Vec<UVec2> = Vec::new();
    let mut drawn_points_with_counts = HashMap::new();

    let mut start = points[0];
    let mut current = start;
    group.push(current);
    drawn_points_with_counts.insert(current, 2);

    while drawn_points_with_counts.len() < points.len() {
        if let Some(p) = points
            .iter()
            .filter(|p| (distance(current.as_vec2(), p.as_vec2()) - 1.0).abs() <= f32::EPSILON)
            .min_by_key(|n| drawn_points_with_counts.get(n).map_or(0, |c| *c))
        {
            current = *p;
            group.push(current);
            if let Some(c) = drawn_points_with_counts.get_mut(p) {
                *c += 1;
            } else {
                drawn_points_with_counts.insert(current, 2);
            }
        }

        // we've traversed and backtracked and we're back at the start without reaching the end of the points
        // so we need to start a collecting the points of a new unconnected object
        if current == start {
            // remove the connecting coordinate
            let _ = group.pop();
            groups.push(group.clone());
            group.clear();
            for val in drawn_points_with_counts.values_mut() {
                *val = 1;
            }

            if let Some(new_start) = points
                .iter()
                .find(|p| !drawn_points_with_counts.contains_key(p))
            {
                start = *new_start;
                current = start;
                group.push(current);
                drawn_points_with_counts.insert(current, 2);
            } else {
                break;
            }
        }
    }
    groups.push(group);

    groups
}
