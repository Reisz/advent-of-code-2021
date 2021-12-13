use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(pub u32, pub u32);

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self(x.parse()?, y.parse()?))
    }
}

#[derive(Clone)]
pub enum Fold {
    Horizontal(u32),
    Vertical(u32),
}

impl Fold {
    pub fn apply_to(&self, pos: Pos) -> Pos {
        match self {
            Self::Horizontal(f) => {
                if pos.1 < *f {
                    pos
                } else {
                    Pos(pos.0, 2 * f - (pos.1))
                }
            }
            Self::Vertical(f) => {
                if pos.0 < *f {
                    pos
                } else {
                    Pos(2 * f - pos.0, pos.1)
                }
            }
        }
    }
}

impl FromStr for Fold {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("fold along ").unwrap();
        let (axis, val) = s.split_once('=').unwrap();
        let val = val.parse()?;
        Ok(match axis {
            "x" => Self::Vertical(val),
            "y" => Self::Horizontal(val),
            _ => panic!("Unknown axis"),
        })
    }
}
