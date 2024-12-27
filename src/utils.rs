use crate::{UVec2, Vec2};

#[inline]
pub fn bounding_box(polygon: impl Iterator<Item = UVec2>) -> Option<(UVec2, UVec2)> {
    polygon
        .map(|p| (p, p))
        .reduce(|(min, max), (a, b)| (min.min(a), max.max(b)))
}

#[inline]
pub fn center_of(polygon: impl Iterator<Item = UVec2>) -> Option<Vec2> {
    bounding_box(polygon)
        .map(|(min, max)| (min.as_vec2(), max.as_vec2()))
        .map(|(min, max)| min + (max - min) / 2.)
}
