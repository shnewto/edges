use crate::Vec2;

// d=√((x2-x1)²+(y2-y1)²)
pub fn distance(a: Vec2, b: Vec2) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
