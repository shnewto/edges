use crate::UVec2;

// Get the bounding box of the polygon
pub fn bounding_box(polygon: &[UVec2]) -> Option<(UVec2, UVec2)> {
    polygon
        .iter()
        .copied()
        .zip(polygon.iter().copied())
        .reduce(|(min, max), (a, b)| (min.min(a), max.max(b)))
}

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

impl Direction {
    pub fn move_point(&self, point: &mut UVec2) {
        match self {
            Direction::North => point.y += 1,
            Direction::South => point.y -= 1,
            Direction::East => point.x += 1,
            Direction::West => point.x -= 1,
            Direction::Northeast => {
                point.x += 1;
                point.y += 1;
            }
            Direction::Northwest => {
                point.x -= 1;
                point.y += 1;
            }
            Direction::Southeast => {
                point.x += 1;
                point.y -= 1;
            }
            Direction::Southwest => {
                point.x -= 1;
                point.y -= 1;
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn handle_neighbors(
    neighbors: u8,
    x_ordering: std::cmp::Ordering,
    y_ordering: std::cmp::Ordering,
) -> Direction {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use Direction::{East, North, Northeast, Northwest, South, Southeast, Southwest, West};
    match neighbors {
        0 | 255 => unreachable!(),
        188..=191 | 127 | 123 | 119 | 115 | 48..=63 => match x_ordering {
            Greater => West,
            Equal => unreachable!(),
            Less => East,
        },
        239 | 238 | 235 | 234 | 223 | 221 | 215 | 213 | 192..=207 => match y_ordering {
            Greater => South,
            Equal => unreachable!(),
            Less => North,
        },
        6 => match x_ordering {
            Greater => Northwest,
            Equal => unreachable!(),
            Less => Southeast,
        },
        9 => match x_ordering {
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
        247 | 245 | 174 | 172 | 170 | 168 | 166 | 164 | 162 | 160 => match x_ordering {
            Greater => North,
            Equal => East,
            Less => unreachable!(),
        },
        253 | 104..=107 | 97 | 96 => match x_ordering {
            Greater => South,
            Equal => East,
            Less => unreachable!(),
        },
        251 | 157 | 156 | 153 | 152 | 149 | 148 | 145 | 144 => match x_ordering {
            Greater => unreachable!(),
            Equal => West,
            Less => North,
        },
        254 | 250 | 80..=87 => match x_ordering {
            Greater => unreachable!(),
            Equal => West,
            Less => South,
        },
        180..=182 => match x_ordering {
            Greater => North,
            Equal => unreachable!(),
            Less => East,
        },
        186 | 184 => match x_ordering {
            Greater => unreachable!(),
            Equal => West,
            Less => East,
        },
        231 | 226 => match x_ordering {
            Greater => North,
            Equal => South,
            Less => unreachable!(),
        },
        236 | 232 => match y_ordering {
            Greater => South,
            Equal => unreachable!(),
            Less => East,
        },
        249 | 248 | 246 | 244 | 240..=242 => match (x_ordering, y_ordering) {
            (Less, Equal) => South,
            (Equal, Less) => East,
            (Greater, Equal) => North,
            (Equal, Greater) => West,
            _ => unreachable!(),
        },

        110 | 103 | 102 => match x_ordering {
            Greater => Northwest,
            Equal => unreachable!(),
            Less => South,
        },
        111 | 109 | 108 | 101 | 100 => match x_ordering {
            Greater => Northwest,
            Equal => East,
            Less => South,
        },
        46 | 44 | 38 | 36 => match x_ordering {
            Greater => Northwest,
            Equal => unreachable!(),
            Less => East,
        },
        43 | 41 | 35 | 33 => match x_ordering {
            Greater => Southwest,
            Equal => unreachable!(),
            Less => East,
        },
        175 | 173 | 171 | 169 | 167 | 165 | 163 | 161 => match x_ordering {
            Greater => North,
            Equal => Southwest,
            Less => East,
        },
        142 | 138 | 134 | 130 => match x_ordering {
            Greater => North,
            Equal => Southeast,
            Less => unreachable!(),
        },
        95 | 93 | 91 | 89 => match x_ordering {
            Greater => West,
            Equal => Northeast,
            Less => unreachable!(),
        },
        141 | 137 | 133 | 129 => match x_ordering {
            Greater => unreachable!(),
            Equal => Southwest,
            Less => North,
        },
        94 | 92 | 90 | 88 => match x_ordering {
            Greater => West,
            Equal => Northeast,
            Less => South,
        },
        23 | 22 | 19 | 18 => match x_ordering {
            Greater => West,
            Equal => unreachable!(),
            Less => Southeast,
        },
        159 | 158 | 155 | 154 | 151 | 150 | 147 | 146 => match x_ordering {
            Greater => North,
            Equal => West,
            Less => Southeast,
        },
        29 | 28 | 25 | 24 => match x_ordering {
            Greater => West,
            Equal => unreachable!(),
            Less => Northeast,
        },
        72..=75 => match x_ordering {
            Greater => South,
            Equal => Northeast,
            Less => unreachable!(),
        },
        68..=71 => match x_ordering {
            Greater => unreachable!(),
            Equal => Northwest,
            Less => South,
        },

        31 | 30 | 27 | 26 => match y_ordering {
            Greater => West,
            Equal => Southeast,
            Less => Northeast,
        },
        76..=79 => match x_ordering {
            Greater => Northwest,
            Equal => Northeast,
            Less => South,
        },
        47 | 45 | 39 | 37 => match y_ordering {
            Greater => Southwest,
            Equal => Northwest,
            Less => East,
        },
        143 | 139 | 135 | 131 => match x_ordering {
            Greater => North,
            Equal => Southwest,
            Less => Southeast,
        },
        10 => match y_ordering {
            Greater | Equal => Southeast,
            Less => Northeast,
        },
        12 => match x_ordering {
            Greater => Northwest,
            Equal => unreachable!(),
            Less => Northeast,
        },
        3 => match x_ordering {
            Greater => Southwest,
            Equal => unreachable!(),
            Less => Southeast,
        },
        5 => match x_ordering {
            Greater => Southwest,
            Equal => unreachable!(),
            Less => Northwest,
        },
        15 => match (x_ordering, y_ordering) {
            (Greater, Less) => Northeast,
            (Greater, Greater) => Northwest,
            (Less, Less) => Southeast,
            (Less, Greater) => Southwest,
            _ => unreachable!(),
        },

        252 | 124..=126 | 120..=122 | 116..=118 | 112..=114 => match x_ordering {
            Greater => West,
            Equal => East,
            Less => South,
        },
        243 | 187 | 185 | 183 | 176..=179 => match x_ordering {
            Greater => North,
            Equal => West,
            Less => East,
        },
        222 | 216..=220 | 214 | 208..=212 => match y_ordering {
            Greater => West,
            Equal => South,
            Less => North,
        },
        237 | 233 | 227..=230 | 225 | 224 => match y_ordering {
            Greater => South,
            Equal => North,
            Less => East,
        },
        7 => match (x_ordering, y_ordering) {
            (Greater, Less) => Northwest,
            (Less, Less) => Southeast,
            (Less, Greater) => Southwest,
            _ => unreachable!(),
        },
        14 | 11 => match (x_ordering, y_ordering) {
            (Greater, Less) => Northeast,
            (Less, Less) => Southeast,
            (Greater, Greater) => Southwest,
            _ => unreachable!(),
        },
        13 => match (x_ordering, y_ordering) {
            (Less, Greater) => Northeast,
            (Less, Less) => Southeast,
            (Greater, Greater) => Southwest,
            _ => unreachable!(),
        },
    }
}
