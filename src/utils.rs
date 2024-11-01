use crate::UVec2;
use std::cmp::Ordering::{Equal, Greater, Less};

pub fn in_polygon(point: UVec2, polygon: &[UVec2]) -> bool {
    let mut is_inside = false;
    for win in polygon.windows(2) {
        let (p1, p2) = (win[0], win[1]);
        if (p1.x.min(p2.x) <= point.x && point.x <= p1.x.max(p2.x))
            && (p1.y.min(p2.y) <= point.y && point.y <= p1.y.max(p2.y))
            && (p1.y.max(p2.y) - p1.y.min(p2.y)) * (point.x - p1.x.min(p2.x))
                == (p2.x.max(p2.x) - p1.x.min(p2.x)) * (point.y - p1.y.min(p2.y))
        {
            return true;
        }

        if p1.y <= point.y && point.y < p2.y || p2.y <= point.y && point.y < p1.y {
            let (point_x, offset_x) = point
                .x
                .checked_sub(p1.x)
                .map_or_else(|| (0, p1.x - point.x), |dx| (dx, 0));
            let (point_y, offset_y) = point
                .y
                .checked_sub(p1.y)
                .map_or_else(|| (0, p1.y - point.y), |dy| (dy, 0));

            let (dx, offset_x) = (p2.x + offset_x)
                .checked_sub(p1.x)
                .map_or_else(|| (0, p1.x - p2.x - offset_x), |dx| (dx, 0));
            let (dy, offset_y) = (p2.y + offset_y)
                .checked_sub(p1.y)
                .map_or_else(|| (0, p1.y - p2.y - offset_y), |dy| (dy, 0));
            if (point_x + offset_x) * dy >= dx * (point_y + offset_y) {
                is_inside = !is_inside;
            }
        }
    }
    is_inside
}

pub fn is_corner(neighbors: u8) -> bool {
    !matches!(
        neighbors,
        255
            | 239
            | 238
            | 232..=235
            | 127
            | 226
            | 223
            | 221
            | 215
            | 213
            | 212
            | 209
            | 207
            | 201
            | 200
            | 196..=198
            | 191..=194
            | 189
            | 187
            | 185
            | 184
            | 177
            | 126
            | 119
            | 118
            | 116
            | 114
            | 58
            | 56
            | 52..=54
            | 48..=50
            | 0
    )
}

#[allow(clippy::too_many_lines)]
pub fn handle_neighbors(current: &mut UVec2, last: UVec2, neighbors: u8) -> Option<UVec2> {
    println!("l:{last}");
    println!("c:{current}");
    println!("n:{neighbors}");
    match neighbors {
        253 | 169..=171 | 40 | 38 | 32 => current.x += 1,
        254 | 85..=87 | 16 => current.x -= 1,
        251 | 110 | 106 | 102 | 64 => current.y -= 1,
        247 | 153 | 157 | 149 | 129 | 128 => current.y += 1,
        233 | 191 | 189 | 187 | 185 | 127 | 126 | 119 | 118 | 58 | 56 | 52..=54 | 48..=50 => {
            match last.x.cmp(&current.x) {
                Greater | Equal => current.x -= 1,
                Less => current.x += 1,
            }
        }
        239 | 238 | 235 | 234 | 223 | 221 | 215 | 213 | 207 | 201 | 200 | 196..=198 | 192..=194 => {
            match last.y.cmp(&current.y) {
                Greater | Equal => current.y -= 1,
                Less => current.y += 1,
            }
        }
        168 | 162 | 160 => match last.x.cmp(&current.x) {
            Greater => current.y += 1,
            Equal => current.x += 1,
            Less => unreachable!(),
        },
        145 | 144 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => current.x -= 1,
            Less => current.y += 1,
        },
        90 | 84 | 80 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => current.x -= 1,
            Less => current.y -= 1,
        },
        98 | 96 => match last.x.cmp(&current.x) {
            Greater => current.y -= 1,
            Equal => current.x += 1,
            Less => unreachable!(),
        },
        177 => match last.x.cmp(&current.x) {
            Greater => {
                current.y += 1;
                return Some(current.with_y(current.y - 1));
            }
            Equal => unreachable!(),
            Less => current.x += 1,
        },
        184 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => {
                current.x -= 1;
                return Some(current.with_x(current.x + 1));
            }
            Less => current.x += 1,
        },
        212 => match last.y.cmp(&current.y) {
            Greater => {
                current.x -= 1;
                return Some(current.with_x(current.x + 1));
            }
            Equal => unreachable!(),
            Less => current.y += 1,
        },
        209 => match last.y.cmp(&current.y) {
            Greater => unreachable!(),
            Equal => {
                current.y -= 1;
                return Some(current.with_y(current.y + 1));
            }
            Less => current.y += 1,
        },
        114 => match last.x.cmp(&current.x) {
            Greater => current.x -= 1,
            Equal => unreachable!(),
            Less => {
                current.y -= 1;
                return Some(current.with_y(current.y + 1));
            }
        },
        116 => match last.x.cmp(&current.x) {
            Greater => current.x -= 1,
            Equal => {
                current.x += 1;
                return Some(current.with_x(current.x - 1));
            }
            Less => unreachable!(),
        },
        232 => match last.y.cmp(&current.y) {
            Greater => current.y -= 1,
            Equal => unreachable!(),
            Less => {
                current.x += 1;
                return Some(current.with_x(current.x - 1));
            }
        },
        226 => match last.y.cmp(&current.y) {
            Greater => current.y -= 1,
            Equal => {
                current.y -= 1;
                return Some(current.with_y(current.y + 1));
            }
            Less => unreachable!(),
        },
        240 => match (last.x.cmp(&current.x), last.y.cmp(&current.y)) {
            (Less, Equal) => current.y -= 1,
            (Equal, Less) => current.x += 1,
            (Greater, Equal) => current.y += 1,
            (Equal, Greater) => current.x -= 1,
            _ => unreachable!(),
        },
        249 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => current.x += 1,
            Less => current.y -= 1,
        },
        115 | 112 => match last.x.cmp(&current.x) {
            Greater => current.x -= 1,
            Equal => current.x += 1,
            Less => current.y -= 1,
        },
        246 => match last.x.cmp(&current.x) {
            Greater => current.y += 1,
            Equal => current.x -= 1,
            Less => current.x += 1,
        },
        186 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => current.x -= 1,
            Less => {
                current.x += 1;
                return Some(current.with_x(current.x - 1));
            }
        },
        231 => match last.x.cmp(&current.x) {
            Greater => current.y += 1,
            Equal => current.y -= 1,
            Less => unreachable!(),
        },
        8 => {
            current.x += 1;
            current.y += 1;
        }
        12 => match last.x.cmp(&current.x) {
            Greater => {
                current.x -= 1;
                current.y -= 1;
            }
            Equal => unreachable!(),
            Less => {
                current.x += 1;
                current.y += 1;
            }
        },
        44 => match last.x.cmp(&current.x) {
            Greater => {
                current.x -= 1;
                current.y -= 1;
            }
            Equal => unreachable!(),
            Less => current.x += 1,
        },
        24 => match last.x.cmp(&current.x) {
            Greater => current.x -= 1,
            Equal => unreachable!(),
            Less => {
                current.x += 1;
                current.y += 1;
            }
        },
        132 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => {
                current.x -= 1;
                current.y -= 1;
            }
            Less => current.y += 1,
        },
        103 => match last.x.cmp(&current.x) {
            Greater => {
                current.x -= 1;
                current.y += 1;
            }
            Equal => unreachable!(),
            Less => current.y -= 1,
        },
        2 => {
            current.x += 1;
            current.y -= 1;
        }
        67 | 65 => match last.x.cmp(&current.x) {
            Greater => unreachable!(),
            Equal => {
                current.x -= 1;
                current.y += 1;
            }
            Less => current.y -= 1,
        },
        3 => match last.x.cmp(&current.x) {
            Greater => {
                current.x -= 1;
                current.y += 1;
            }
            Equal => unreachable!(),
            Less => {
                current.x += 1;
                current.y -= 1;
            }
        },
        22 | 18 => match last.x.cmp(&current.x) {
            Greater => current.x -= 1,
            Equal => unreachable!(),
            Less => {
                current.x += 1;
                current.y -= 1;
            }
        },
        7 => match (last.x.cmp(&current.x), last.y.cmp(&current.y)) {
            (Greater, Less) => {
                current.x -= 1;
                current.y += 1;
            }
            (Less, Greater) => {
                current.x -= 1;
                current.y -= 1;
            }
            (Less, Less) => {
                current.x += 1;
                current.y -= 1;
            }
            _ => unreachable!(),
        },
        0 | 255 => unreachable!(),
        _ => {
            unreachable!()
        }
    }
    None
}
