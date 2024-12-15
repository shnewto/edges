use crate::UVec2;

// Get the bounding box of the polygon
pub fn bounding_box(polygon: &[UVec2]) -> Option<(UVec2, UVec2)> {
    polygon
        .iter()
        .copied()
        .zip(polygon.iter().copied())
        .reduce(|(min, max), (a, b)| (min.min(a), max.max(b)))
}

pub fn in_polygon(x: u32, y: u32, polygon: &[UVec2]) -> bool {
    if let Some((min, max)) = bounding_box(polygon) {
        // Check if the is within the bounding box
        if x < min.x || x > max.x || y < min.y || y > max.y {
            return false; // Early exit if outside the bounding box
        }
    }

    let mut is_inside = false;

    for (p1, p2) in polygon.windows(2).map(|win| (win[0], win[1])) {
        let (min, max) = (p1.min(p2), p1.max(p2));
        let (dy, dx) = (max.y - min.y, max.x - min.x);

        if min.y <= y && y < max.y && x <= min.x + dx * (y - min.y) / dy {
            if min.x <= x && x < max.x {
                return true;
            }
            is_inside = !is_inside;
        }
    }
    is_inside
}