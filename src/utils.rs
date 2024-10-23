pub type Point = (u32, u32);

// d=√((x2-x1)²+(y2-y1)²)
pub fn distance((x2, y2): Point, (x1, y1): Point) -> f32 {
    ((x2 as f32 - x1 as f32).powi(2) + (y2 as f32 - y1 as f32).powi(2)).sqrt()
}
