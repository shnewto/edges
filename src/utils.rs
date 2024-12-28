use crate::{UVec2, Vec2};

#[inline]
pub fn bounding_box(polygon: impl Iterator<Item = UVec2>) -> Option<(UVec2, UVec2)> {
    polygon
        .map(|p| (p, p))
        .reduce(|(min, max), (a, b)| (min.min(a), max.max(b)))
}

#[inline]
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn center_of(polygon: &[UVec2]) -> Option<Vec2> {
    polygon
        .iter()
        .copied()
        .reduce(|acc, p| acc + p)
        .map(|sum| (sum / polygon.len() as u32).as_vec2())
}
