use crate::UVec2;
use std::cmp::Ordering::{Equal, Greater, Less};

/// Calculates the bounding box of the given polygon.
///
/// # Arguments
///
/// * `polygon` - A slice of `UVec2` points representing the vertices of the polygon.
///
/// # Returns
///
/// Returns an `Option<(UVec2, UVec2)>` where the first `UVec2` is the minimum point (bottom-left)
/// and the second `UVec2` is the maximum point (top-right) of the bounding box. Returns `None`
/// if the polygon is empty.
fn bounding_box(polygon: &[UVec2]) -> Option<(UVec2, UVec2)> {
    polygon
        .iter()
        .copied()
        .zip(polygon.iter().copied())
        .reduce(|(min, max), (a, b)| (min.min(a), max.max(b)))
}

/// Determines if a given point is inside a polygon using the ray-casting algorithm.
///
/// # Arguments
///
/// * `point` - A `UVec2` representing the point to check.
/// * `polygon` - A slice of `UVec2` points representing the vertices of the polygon.
///
/// # Returns
///
/// Returns `true` if the point is inside the polygon, and `false` if it is outside or on the edge.
pub fn in_polygon(point: UVec2, polygon: &[UVec2]) -> bool {
    if let Some((min, max)) = bounding_box(polygon) {
        // Check if the point is within the bounding box
        if point.x < min.x || point.x > max.x || point.y < min.y || point.y > max.y {
            return false; // Early exit if outside the bounding box
        }
    }

    let mut is_inside = false;

    for i in 0..polygon.len() {
        let (p1, p2) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        let (min, max) = (p1.min(p2), p1.max(p2));
        let (dy, dx) = (max.y - min.y, max.x - min.x);

        if min.y <= point.y && point.y < max.y && point.x <= min.x + dx * (point.y - min.y) / dy {
            if min.x <= point.x && point.x < max.x {
                return true;
            }
            is_inside = !is_inside;
        }
    }
    is_inside
}

/// Checks if the given neighbors state indicates a corner pixel.
///
/// # Arguments
///
/// * `neighbors` - A byte representing the state of neighboring pixels.
///
/// # Returns
///
/// Returns `true` if the pixel is a corner, and `false` otherwise.
pub fn is_corner(neighbors: u8) -> bool {
    !matches!(
        neighbors,
        255
            | 239
            | 238
            | 235
            | 234
            | 223
            | 221
            | 215
            | 213
            | 188..=207
            | 127
            | 123
            | 119
            | 115
            | 48..=63
            | 9
            | 6
            | 0
    )
}

pub enum Direction {
    North,
    South,
    East,
    West,

    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

/// Handles the neighbor states and determines the direction based on the current and last positions.
///
/// # Arguments
///
/// * `current` - A `UVec2` representing the current position.
/// * `last` - A `UVec2` representing the last position.
/// * `neighbors` - A byte representing the state of neighboring pixels.
///
/// # Returns
///
/// Returns a `Direction` indicating the direction of movement based on the neighbor states.
#[allow(clippy::too_many_lines)]
pub fn handle_neighbors(current: UVec2, last: UVec2, neighbors: u8) -> Direction {
    use Direction::{East, North, Northeast, Northwest, South, Southeast, Southwest, West};
    match neighbors {
        0 | 255 => unreachable!(),
        188..=191 | 127 | 123 | 119 | 115 | 48..=63 => match last.x.cmp(&current.x) {
            Greater => West,
            Equal => unreachable!(),
            Less => East,
        },
        239 | 238 | 235 | 234 | 223 | 221 | 215 | 213 | 192..=207 => match last.y.cmp(&current.y) {
            Greater => South,
            Equal => unreachable!(),
            Less => North,
        },
        6 => match last.x.cmp(&current.x) {
            Greater => Northwest,
            Equal => unreachable!(),
            Less => Southeast,
        },
        9 => match last.x.cmp(&current.x) {
            Greater => Southwest,
            Equal => unreachable!(),
            Less => Northeast,
        },

        140 | 136 | 132 | 128 => North,
        99 | 98 | 64..=67 => South,
        42 | 40 | 34 | 32 => East,
        21 | 20 | 17 | 16 => West,
        8 => Northeast,
        4 => Northwest,
        2 => Southeast,
        1 => Southwest,
        247 | 245 | 174 | 172 | 170 | 168 | 166 | 164 | 162 | 160 => match last.x.cmp(&current.x) {
            Greater => North,
            Equal => East,
            Less => unreachable!(),
        },
        253 | 104..=107 | 97 | 96 => match last.x.cmp(&current.x) {
            Greater => South,
            Equal => East,
            Less => unreachable!(),
        },
        251 | 157 | 156 | 153 | 152 | 149 | 148 | 145 | 144 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => West,
            Less => North,
        },
        254 | 250 | 80..=87 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => West,
            Less => South,
        },
        180..=182 => match last.x.cmp(&current.x) {
            Greater => North,
            Equal => unreachable!(),
            Less => East,
        },
        186 | 184 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => West,
            Less => East,
        },
        231 | 226 => match last.x.cmp(&current.x) {
            Greater => North,
            Equal => South,
            Less => unreachable!(),
        },
        236 | 232 => match last.y.cmp(&current.y) {
            Greater => South,
            Equal => unreachable!(),
            Less => East,
        },
        249 | 248 | 246 | 244 | 240..=242 => {
            match (last.x.cmp(&current.x), last.y.cmp(&current.y)) {
                (Less, Equal) => South,
                (Equal, Less) => East,
                (Greater, Equal) => North,
                (Equal, Greater) => West,
                _ => unreachable!(),
            }
        }

        110 | 103 | 102 => match last.x.cmp(&current.x) {
            Greater => Northwest,
            Equal => unreachable!(),
            Less => South,
        },
        111 | 109 | 108 | 101 | 100 => match last.x.cmp(&current.x) {
            Greater => Northwest,
            Equal => East,
            Less => South,
        },
        46 | 44 | 38 | 36 => match last.x.cmp(&current.x) {
            Greater => Northwest,
            Equal => unreachable!(),
            Less => East,
        },
        43 | 41 | 35 | 33 => match last.x.cmp(&current.x) {
            Greater => Southwest,
            Equal => unreachable!(),
            Less => East,
        },
        175 | 173 | 171 | 169 | 167 | 165 | 163 | 161 => match last.x.cmp(&current.x) {
            Greater => North,
            Equal => Southwest,
            Less => East,
        },
        142 | 138 | 134 | 130 => match last.x.cmp(&current.x) {
            Greater => North,
            Equal => Southeast,
            Less => unreachable!(),
        },
        95 | 93 | 91 | 89 => match last.x.cmp(&current.x) {
            Greater => West,
            Equal => Northeast,
            Less => unreachable!(),
        },
        141 | 137 | 133 | 129 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => Southwest,
            Less => North,
        },
        94 | 92 | 90 | 88 => match last.x.cmp(&current.x) {
            Greater => West,
            Equal => Northeast,
            Less => South,
        },
        23 | 22 | 19 | 18 => match last.x.cmp(&current.x) {
            Greater => West,
            Equal => unreachable!(),
            Less => Southeast,
        },
        159 | 158 | 155 | 154 | 151 | 150 | 147 | 146 => match last.x.cmp(&current.x) {
            Greater => North,
            Equal => West,
            Less => Southeast,
        },
        29 | 28 | 25 | 24 => match last.x.cmp(&current.x) {
            Greater => West,
            Equal => unreachable!(),
            Less => Northeast,
        },
        72..=75 => match last.x.cmp(&current.x) {
            Greater => South,
            Equal => Northeast,
            Less => unreachable!(),
        },
        68..=71 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => Northwest,
            Less => South,
        },

        31 | 30 | 27 | 26 => match last.y.cmp(&current.y) {
            Greater => West,
            Equal => Southeast,
            Less => Northeast,
        },
        76..=79 => match last.x.cmp(&current.x) {
            Greater => Northwest,
            Equal => Northeast,
            Less => South,
        },
        47 | 45 | 39 | 37 => match last.y.cmp(&current.y) {
            Greater => Southwest,
            Equal => Northwest,
            Less => East,
        },
        143 | 139 | 135 | 131 => match last.x.cmp(&current.x) {
            Greater => North,
            Equal => Southwest,
            Less => Southeast,
        },
        10 => match last.y.cmp(&current.y) {
            Greater | Equal => Southeast,
            Less => Northeast,
        },
        12 => match last.x.cmp(&current.x) {
            Greater => Northwest,
            Equal => unreachable!(),
            Less => Northeast,
        },
        3 => match last.x.cmp(&current.x) {
            Greater => Southwest,
            Equal => unreachable!(),
            Less => Southeast,
        },
        5 => match last.x.cmp(&current.x) {
            Greater => Southwest,
            Equal => unreachable!(),
            Less => Northwest,
        },
        15 => match (last.x.cmp(&current.x), last.y.cmp(&current.y)) {
            (Greater, Less) => Northeast,
            (Greater, Greater) => Northwest,
            (Less, Less) => Southeast,
            (Less, Greater) => Southwest,
            _ => unreachable!(),
        },

        252 | 124..=126 | 120..=122 | 116..=118 | 112..=114 => match last.x.cmp(&current.x) {
            Greater => West,
            Equal => East,
            Less => South,
        },
        243 | 187 | 185 | 183 | 176..=179 => match last.x.cmp(&current.x) {
            Greater => North,
            Equal => West,
            Less => East,
        },
        222 | 216..=220 | 214 | 208..=212 => match last.y.cmp(&current.y) {
            Greater => West,
            Equal => South,
            Less => North,
        },
        237 | 233 | 227..=230 | 225 | 224 => match last.y.cmp(&current.y) {
            Greater => South,
            Equal => North,
            Less => East,
        },
        7 => match (last.x.cmp(&current.x), last.y.cmp(&current.y)) {
            (Greater, Less) => Northwest,
            (Less, Less) => Southeast,
            (Less, Greater) => Southwest,
            _ => unreachable!(),
        },
        14 | 11 => match (last.x.cmp(&current.x), last.y.cmp(&current.y)) {
            (Greater, Less) => Northeast,
            (Less, Less) => Southeast,
            (Greater, Greater) => Southwest,
            _ => unreachable!(),
        },
        13 => match (last.x.cmp(&current.x), last.y.cmp(&current.y)) {
            (Less, Greater) => Northeast,
            (Less, Less) => Southeast,
            (Greater, Greater) => Southwest,
            _ => unreachable!(),
        },
    }
}
