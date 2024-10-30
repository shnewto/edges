use crate::UVec2;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{AddAssign, SubAssign};

pub fn is_corner(neighbors: u8) -> bool {
    !matches!(
        neighbors,
        213 | 234
            | 200
            | 196
            | 194
            | 193
            | 192
            | 185
            | 118
            | 56
            | 52
            | 50
            | 49
            | 48
            | 177
            | 170
            | 184
            | 212
            | 209
            | 114
            | 116
            | 232
            | 226
            | 251
            | 255
            | 0
    )
}

#[allow(clippy::too_many_lines)]
pub fn match_neighbors(neighbors: u8, current: &mut UVec2, group: &mut Vec<UVec2>) {
    if let Some(last) = group.last() {
        let last = *last;
        if last != *current && is_corner(neighbors) {
            group.push(*current);
        }
        println!("l:{last}");
        println!("c:{current}");
        println!("n:{neighbors}");
        // println!("{group:#?}");
        if current.x == 511 {
            todo!();
        }
        match neighbors {
            189 | 191 | 119 | 126 | 187 | 185 | 118 | 56 | 52 | 50 | 49 | 48 => {
                match last.x.cmp(&current.x) {
                    Greater | Equal => current.x.sub_assign(1),
                    Less => current.x.add_assign(1),
                }
            }
            221 | 215 | 238 | 235 | 213 | 234 | 200 | 196 | 194 | 193 | 192 => {
                match last.y.cmp(&current.y) {
                    Greater | Equal => current.y.sub_assign(1),
                    Less => current.y.add_assign(1),
                }
            }
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
                    group.push(*current);
                    current.y += 1;
                }
                Equal => unreachable!(),
                Less => current.x.add_assign(1),
            },
            184 => match last.x.cmp(&current.x) {
                Greater => unreachable!(),
                Equal => {
                    group.push(*current);
                    current.x -= 1;
                }
                Less => current.x.add_assign(1),
            },
            212 => match last.y.cmp(&current.y) {
                Greater => {
                    group.push(*current);
                    current.x -= 1;
                }
                Equal => unreachable!(),
                Less => current.y.add_assign(1),
            },
            209 => match last.y.cmp(&current.y) {
                Greater => unreachable!(),
                Equal => {
                    group.push(*current);
                    current.y -= 1;
                }
                Less => current.y.add_assign(1),
            },
            114 => match last.x.cmp(&current.x) {
                Greater => current.x.sub_assign(1),
                Equal => unreachable!(),
                Less => {
                    group.push(*current);
                    current.y -= 1;
                }
            },
            116 => match last.x.cmp(&current.x) {
                Greater => current.x.sub_assign(1),
                Equal => {
                    group.push(*current);
                    current.x += 1;
                }
                Less => unreachable!(),
            },
            232 => match last.y.cmp(&current.y) {
                Greater => current.y.sub_assign(1),
                Equal => unreachable!(),
                Less => {
                    group.push(*current);
                    current.x += 1;
                }
            },
            226 => match last.y.cmp(&current.y) {
                Greater => current.y.sub_assign(1),
                Equal => {
                    group.push(*current);
                    current.y -= 1;
                }
                Less => unreachable!(),
            },
            169..=171 | 253 => match last.x.cmp(&current.x) {
                Greater | Less => unreachable!(),
                Equal => {
                    group.push(*current);
                    current.x += 1;
                }
            },
            153 | 157 | 149 => match last.x.cmp(&current.x) {
                Greater | Equal => unreachable!(),
                Less => {
                    group.push(*current);
                    current.y += 1;
                }
            },
            85..=87 | 254 => match last.x.cmp(&current.x) {
                Greater | Less => unreachable!(),
                Equal => {
                    group.push(*current);
                    current.x -= 1;
                }
            },
            251 => match last.x.cmp(&current.x) {
                Greater | Equal => unreachable!(),
                Less => {
                    group.push(*current);
                    current.y -= 1;
                }
            },
            247 => match last.x.cmp(&current.x) {
                Greater => {
                    group.push(*current);
                    current.y += 1;
                }
                Equal | Less => unreachable!(),
            },
            110 | 102 => match last.x.cmp(&current.x) {
                Greater => {
                    group.push(*current);
                    current.y -= 1;
                }
                Equal | Less => unreachable!(),
            },
            106 => match last.x.cmp(&current.x) {
                Greater | Equal => {
                    group.push(*current);
                    current.y -= 1;
                }
                Less => unreachable!(),
            },
            _ => {}
        }
    }
}
