#[cfg(feature = "parallel")]
use rayon::prelude::*;

use super::Neighbors;
use crate::UVec2;
use Direction::{East, North, Northeast, Northwest, South, Southeast, Southwest, West};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn find_in(self, current: UVec2, points: &[UVec2]) -> Option<UVec2> {
        #[cfg(not(feature = "parallel"))]
        let iter = points.iter();
        #[cfg(feature = "parallel")]
        let iter = points.par_iter();
        match self {
            Direction::North => iter
                .filter(|p| p.x == current.x && p.y > current.y)
                .min_by_key(|p| p.y),
            Direction::South => iter
                .filter(|p| p.x == current.x && p.y < current.y)
                .max_by_key(|p| p.y),
            Direction::East => iter
                .filter(|p| p.y == current.y && p.x > current.x)
                .min_by_key(|p| p.x),
            Direction::West => iter
                .filter(|p| p.y == current.y && p.x < current.x)
                .max_by_key(|p| p.x),
            Direction::Northeast => iter
                .filter(|p| {
                    p.cmpgt(current).all() && p.y.abs_diff(current.y) == p.x.abs_diff(current.x)
                })
                .min_by_key(|p| p.y),
            Direction::Northwest => iter
                .filter(|p| {
                    (p.x < current.x && p.y > current.y)
                        && p.y.abs_diff(current.y) == p.x.abs_diff(current.x)
                })
                .min_by_key(|p| p.y),
            Direction::Southeast => iter
                .filter(|p| {
                    (p.x > current.x && p.y < current.y)
                        && p.y.abs_diff(current.y) == p.x.abs_diff(current.x)
                })
                .max_by_key(|p| p.y),
            Direction::Southwest => iter
                .filter(|p| {
                    p.cmplt(current).all() && p.y.abs_diff(current.y) == p.x.abs_diff(current.x)
                })
                .max_by_key(|p| p.y),
        }
        .copied()
    }

    #[inline]
    pub fn reverse(self) -> Self {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
            Northeast => Southwest,
            Northwest => Southeast,
            Southeast => Northwest,
            Southwest => Northeast,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn next_direction(previous_direction: Option<Self>, neighbors: Neighbors) -> Self {
        match neighbors.bits() {
            140 | 136 | 132 | 128 => Some(North),
            64..=67 => Some(South),
            42 | 40 | 34 | 32 => Some(East),
            21 | 20 | 17 | 16 => Some(West),
            8 => Some(Northeast),
            4 => Some(Northwest),
            2 => Some(Southeast),
            1 => Some(Southwest),

            239
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
            | 6 => previous_direction,

            80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 254 => match previous_direction {
                Some(North) => Some(West),
                Some(East) => Some(South),
                _ => None,
            },
            96 | 97 | 98 | 99 | 104 | 105 | 106 | 107 | 253 => match previous_direction {
                Some(North) => Some(East),
                Some(West) => Some(South),
                _ => None,
            },
            144 | 145 | 148 | 149 | 152 | 153 | 156 | 157 | 251 => match previous_direction {
                Some(South) => Some(West),
                Some(East) => Some(North),
                _ => None,
            },
            160 | 162 | 164 | 166 | 168 | 170 | 172 | 174 | 247 => match previous_direction {
                Some(South) => Some(East),
                Some(West) => Some(North),
                _ => None,
            },

            18 | 19 | 22 | 23 => match previous_direction {
                Some(Northwest) => Some(West),
                Some(East) => Some(Southeast),
                _ => None,
            },
            24 | 25 | 28 | 29 => match previous_direction {
                Some(East) => Some(Northeast),
                Some(Southwest) => Some(West),
                _ => None,
            },
            33 | 35 | 41 | 43 => match previous_direction {
                Some(West) => Some(Southwest),
                Some(Northeast) => Some(East),
                _ => None,
            },
            36 | 38 | 44 | 46 => match previous_direction {
                Some(West) => Some(Northwest),
                Some(Southeast) => Some(East),
                _ => None,
            },
            68..=71 => match previous_direction {
                Some(North) => Some(Northwest),
                Some(Southeast) => Some(South),
                _ => None,
            },
            72..=75 => match previous_direction {
                Some(North) => Some(Northeast),
                Some(Southwest) => Some(South),
                _ => None,
            },
            129 | 133 | 137 | 141 => match previous_direction {
                Some(South) => Some(Southwest),
                Some(Northeast) => Some(North),
                _ => None,
            },
            130 | 134 | 138 | 142 => match previous_direction {
                Some(South) => Some(Southeast),
                Some(Northwest) => Some(North),
                _ => None,
            },

            3 => match previous_direction {
                Some(Northeast) => Some(Southeast),
                Some(Northwest) => Some(Southwest),
                _ => None,
            },
            5 => match previous_direction {
                Some(Northeast) => Some(Northwest),
                Some(Southeast) => Some(Southwest),
                _ => None,
            },
            10 => match previous_direction {
                Some(Northwest) => Some(Northeast),
                Some(Southwest) => Some(Southeast),
                _ => None,
            },
            12 => match previous_direction {
                Some(Southeast) => Some(Northeast),
                Some(Southwest) => Some(Northwest),
                _ => None,
            },

            7 => match previous_direction {
                Some(Northeast) => Some(Southeast),
                Some(Northwest) => Some(Northwest),
                Some(Southeast) => Some(Southwest),
                _ => None,
            },
            11 => match previous_direction {
                Some(Northeast) => Some(Southeast),
                Some(Northwest) => Some(Northeast),
                Some(Southwest) => Some(Southwest),
                _ => None,
            },
            13 => match previous_direction {
                Some(Northeast) => Some(Northeast),
                Some(Southeast) => Some(Southwest),
                Some(Southwest) => Some(Northwest),
                _ => None,
            },
            14 => match previous_direction {
                Some(Northwest) => Some(Northeast),
                Some(Southeast) => Some(Southeast),
                Some(Southwest) => Some(Northwest),
                _ => None,
            },

            112..=114 | 116..=118 | 120..=122 | 124..=126 | 252 => match previous_direction {
                Some(North) => Some(East),
                Some(East) => Some(South),
                Some(West) => Some(West),
                _ => None,
            },
            176..=187 | 243 => match previous_direction {
                Some(South) => Some(West),
                Some(East) => Some(East),
                Some(West) => Some(North),
                _ => None,
            },
            208..=212 | 214 | 216..=220 | 222 | 250 => match previous_direction {
                Some(North) => Some(North),
                Some(South) => Some(West),
                Some(East) => Some(South),
                _ => None,
            },
            224..=233 | 237 | 236 | 245 => match previous_direction {
                Some(North) => Some(East),
                Some(South) => Some(South),
                Some(West) => Some(North),
                _ => None,
            },

            88..=95 => match previous_direction {
                Some(North) => Some(Northeast),
                Some(East) => Some(South),
                Some(Southwest) => Some(West),
                _ => None,
            },
            100..=103 | 108..=111 => match previous_direction {
                Some(North) => Some(East),
                Some(West) => Some(Northwest),
                Some(Southeast) => Some(South),
                _ => None,
            },
            146 | 147 | 150 | 151 | 154 | 155 | 158 | 159 => match previous_direction {
                Some(South) => Some(West),
                Some(East) => Some(Southeast),
                Some(Northwest) => Some(North),
                _ => None,
            },
            161 | 163 | 165 | 167 | 169 | 171 | 173 | 175 => match previous_direction {
                Some(South) => Some(Southwest),
                Some(West) => Some(North),
                Some(Northeast) => Some(East),
                _ => None,
            },

            26 | 27 | 30 | 31 => match previous_direction {
                Some(East) => Some(Southeast),
                Some(Northwest) => Some(Northeast),
                Some(Southwest) => Some(West),
                _ => None,
            },
            37 | 39 | 45 | 47 => match previous_direction {
                Some(West) => Some(Northwest),
                Some(Northeast) => Some(East),
                Some(Southeast) => Some(Southwest),
                _ => None,
            },
            76..=79 => match previous_direction {
                Some(North) => Some(Northeast),
                Some(Southeast) => Some(South),
                Some(Southwest) => Some(Northwest),
                _ => None,
            },
            131 | 135 | 139 | 143 => match previous_direction {
                Some(South) => Some(Southwest),
                Some(Northeast) => Some(Southeast),
                Some(Northwest) => Some(North),
                _ => None,
            },

            240 | 241 | 242 | 244 | 246 | 248 | 249 => match previous_direction {
                Some(North) => Some(East),
                Some(South) => Some(West),
                Some(East) => Some(South),
                Some(West) => Some(North),
                _ => None,
            },
            15 => match previous_direction {
                Some(Northeast) => Some(Southeast),
                Some(Northwest) => Some(Northeast),
                Some(Southeast) => Some(Southwest),
                Some(Southwest) => Some(Northwest),
                _ => None,
            },
            0 | 255 => unreachable!(),
        }
        .unwrap_or({
            if neighbors.contains(Neighbors::NORTH) {
                North
            } else if neighbors.contains(Neighbors::SOUTH) {
                South
            } else if neighbors.contains(Neighbors::EAST) {
                East
            } else if neighbors.contains(Neighbors::WEST) {
                West
            } else if neighbors.contains(Neighbors::NORTHEAST) {
                Northeast
            } else if neighbors.contains(Neighbors::NORTHWEST) {
                Northwest
            } else if neighbors.contains(Neighbors::SOUTHEAST) {
                Southeast
            } else if neighbors.contains(Neighbors::SOUTHWEST) {
                Southwest
            } else {
                unreachable!()
            }
        })
    }
}
