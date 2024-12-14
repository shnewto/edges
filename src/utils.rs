use crate::UVec2;

// Get the bounding box of the polygon
pub fn bounding_box(polygon: impl Iterator<Item = UVec2>) -> Option<(UVec2, UVec2)> {
    polygon
        .map(|p| (p, p))
        .reduce(|(min, max), (a, b)| (min.min(a), max.max(b)))
}

#[inline]
#[must_use]
pub fn center(polygon: impl Iterator<Item = UVec2>) -> Option<UVec2> {
    bounding_box(polygon).map(|(min, max)| min + (max - min) / 2)
}
