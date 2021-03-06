use std::{
    cmp::{max, min},
    str::FromStr,
};

use crate::pos::Pos;

use anyhow::anyhow;

fn to_range(a: u32, b: u32) -> impl Iterator<Item = u32> {
    let min = min(a, b);
    let max = max(a, b);
    let iter = min..=max;
    if min == b {
        return iter.rev().collect::<Vec<_>>().into_iter();
    }
    iter.collect::<Vec<_>>().into_iter()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line(pub Pos, pub Pos);

impl Line {
    pub fn points(&self) -> Vec<Pos> {
        if self.0.x == self.1.x {
            let x = self.0.x;
            to_range(self.0.y, self.1.y).map(|y| Pos { x, y }).collect()
        } else if self.0.y == self.1.y {
            let y = self.0.y;
            to_range(self.0.x, self.1.x).map(|x| Pos { x, y }).collect()
        } else {
            to_range(self.0.x, self.1.x)
                .zip(to_range(self.0.y, self.1.y))
                .map(|(x, y)| Pos { x, y })
                .collect()
        }
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once("->")
            .ok_or(anyhow!("Could not find split point."))?;
        Ok(Line(a.trim().parse()?, b.trim().parse()?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_line_points() -> Result<()> {
        assert_eq!(
            "1,1 -> 1,3".parse::<Line>()?.points(),
            vec![Pos { x: 1, y: 1 }, Pos { x: 1, y: 2 }, Pos { x: 1, y: 3 }]
        );
        assert_eq!(
            "9,7 -> 7,7".parse::<Line>()?.points(),
            vec![Pos { x: 9, y: 7 }, Pos { x: 8, y: 7 }, Pos { x: 7, y: 7 }]
        );
        assert_eq!(
            "1,1 -> 3,3".parse::<Line>()?.points(),
            vec![Pos { x: 1, y: 1 }, Pos { x: 2, y: 2 }, Pos { x: 3, y: 3 }]
        );
        assert_eq!(
            "9,7 -> 7,9".parse::<Line>()?.points(),
            vec![Pos { x: 9, y: 7 }, Pos { x: 8, y: 8 }, Pos { x: 7, y: 9 }]
        );
        Ok(())
    }
}
