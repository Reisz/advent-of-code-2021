use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Pos {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or(anyhow!("Could not find split point."))?;
        Ok(Pos {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}
