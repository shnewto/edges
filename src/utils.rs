use crate::{Edges, UVec2};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{AddAssign, SubAssign};

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

impl Edges {
    #[allow(clippy::too_many_lines)]
    pub(super) fn collect_object(&self, mut current: UVec2) -> Vec<UVec2> {
        let mut group: Vec<UVec2> = Vec::new();
        group.push(current);
        loop {
            let (last, neighbors) = (*group.last().unwrap(), self.image.get_neighbors(current));
            if last != current && is_corner(neighbors) {
                group.push(current);
            }
            match neighbors {
                253 | 169..=171 | 32 => current.x.add_assign(1),
                254 | 85..=87 | 16 => current.x.sub_assign(1),
                251 | 110 | 106 | 102 | 64 => current.y.sub_assign(1),
                247 | 153 | 157 | 149 | 128 => current.y.add_assign(1),
                233
                | 191
                | 189
                | 187
                | 185
                | 127
                | 126
                | 119
                | 118
                | 58
                | 56
                | 52..=54
                | 48..=50 => match last.x.cmp(&current.x) {
                    Greater | Equal => current.x.sub_assign(1),
                    Less => current.x.add_assign(1),
                },
                239
                | 238
                | 235
                | 234
                | 223
                | 221
                | 215
                | 213
                | 207
                | 201
                | 200
                | 196..=198
                | 192..=194 => match last.y.cmp(&current.y) {
                    Greater | Equal => current.y.sub_assign(1),
                    Less => current.y.add_assign(1),
                },
                168 | 160 => match last.x.cmp(&current.x) {
                    Greater => current.y.add_assign(1),
                    Equal => current.x.add_assign(1),
                    Less => unreachable!(),
                },
                145 | 144 => match last.x.cmp(&current.x) {
                    Greater => unreachable!(),
                    Equal => current.x.sub_assign(1),
                    Less => current.y.add_assign(1),
                },
                84 | 80 => match last.x.cmp(&current.x) {
                    Greater => unreachable!(),
                    Equal => current.x.sub_assign(1),
                    Less => current.y.sub_assign(1),
                },
                98 | 96 => match last.x.cmp(&current.x) {
                    Greater => current.y.sub_assign(1),
                    Equal => current.x.add_assign(1),
                    Less => unreachable!(),
                },
                177 => match last.x.cmp(&current.x) {
                    Greater => {
                        group.push(current);
                        current.y += 1;
                    }
                    Equal => unreachable!(),
                    Less => current.x.add_assign(1),
                },
                184 => match last.x.cmp(&current.x) {
                    Greater => unreachable!(),
                    Equal => {
                        group.push(current);
                        current.x -= 1;
                    }
                    Less => current.x.add_assign(1),
                },
                212 => match last.y.cmp(&current.y) {
                    Greater => {
                        group.push(current);
                        current.x -= 1;
                    }
                    Equal => unreachable!(),
                    Less => current.y.add_assign(1),
                },
                209 => match last.y.cmp(&current.y) {
                    Greater => unreachable!(),
                    Equal => {
                        group.push(current);
                        current.y -= 1;
                    }
                    Less => current.y.add_assign(1),
                },
                114 => match last.x.cmp(&current.x) {
                    Greater => current.x.sub_assign(1),
                    Equal => unreachable!(),
                    Less => {
                        group.push(current);
                        current.y -= 1;
                    }
                },
                116 => match last.x.cmp(&current.x) {
                    Greater => current.x.sub_assign(1),
                    Equal => {
                        group.push(current);
                        current.x += 1;
                    }
                    Less => unreachable!(),
                },
                232 => match last.y.cmp(&current.y) {
                    Greater => current.y.sub_assign(1),
                    Equal => unreachable!(),
                    Less => {
                        group.push(current);
                        current.x += 1;
                    }
                },
                226 => match last.y.cmp(&current.y) {
                    Greater => current.y.sub_assign(1),
                    Equal => {
                        group.push(current);
                        current.y -= 1;
                    }
                    Less => unreachable!(),
                },
                240 => match (last.x.cmp(&current.x), last.y.cmp(&current.y)) {
                    (Less, Equal) => current.y.sub_assign(1),
                    (Equal, Less) => current.x.add_assign(1),
                    (Greater, Equal) => current.y.add_assign(1),
                    (Equal, Greater) => current.x.sub_assign(1),
                    _ => unreachable!(),
                },
                249 => match last.x.cmp(&current.x) {
                    Greater => unreachable!(),
                    Equal => current.x.add_assign(1),
                    Less => current.y.sub_assign(1),
                },
                112 => match last.x.cmp(&current.x) {
                    Greater => current.x.sub_assign(1),
                    Equal => current.x.add_assign(1),
                    Less => current.y.sub_assign(1),
                },
                246 => match last.x.cmp(&current.x) {
                    Greater => current.y.add_assign(1),
                    Equal => current.x.sub_assign(1),
                    Less => current.x.add_assign(1),
                },
                0 | 255 => unreachable!(),
                _ => {
                    println!("l:{last}");
                    println!("c:{current}");
                    println!("n:{neighbors}");
                    unreachable!()
                }
            }
            if current == group[0] {
                break group;
            }
        }
    }
}
