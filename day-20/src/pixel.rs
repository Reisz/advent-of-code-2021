use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use util::grid::{Digit, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
    Dark,
    Light,
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::Dark
    }
}

impl TryFrom<char> for Pixel {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Pixel::Dark,
            '#' => Pixel::Light,
            c => return Err(anyhow!("Unknown character {}", c)),
        })
    }
}

impl Digit for Pixel {
    fn from_char(c: char) -> Option<Self> {
        c.try_into().ok()
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Dark => '.',
                Pixel::Light => '#',
            }
        )
    }
}

pub type Mapping = [Pixel; 512];

#[derive(Clone)]
pub struct Pixels(Grid<Pixel>, Pixel);

impl Pixels {
    fn filter_index(&self, x: isize, y: isize, free_space: Pixel) -> usize {
        let mut index = 0;
        for (xoff, yoff) in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
            index = (index << 1)
                | (*self.0.get(x + xoff, y + yoff).unwrap_or(&free_space) == Pixel::Light) as usize;
        }

        index
    }

    pub fn apply_filter(&self, mapping: &Mapping) -> Self {
        //assert!(mapping[0] == Pixel::Dark);
        let mut result = Grid::new(self.0.width() + 2, self.0.height() + 2);

        for x in 0..result.width() {
            for y in 0..result.height() {
                *result.get_mut(x, y).unwrap() = mapping[self.filter_index(x - 1, y - 1, self.1)];
            }
        }

        let index = if self.1 == Pixel::Dark {
            0
        } else {
            0b1_1111_1111
        };

        Pixels(result, mapping[index])
    }

    pub fn count_lights(&self) -> usize {
        assert_eq!(self.1, Pixel::Dark);
        let mut result = 0;
        self.0
            .for_each(|pix| result += (*pix == Pixel::Light) as usize);
        result
    }
}

impl FromStr for Pixels {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pixels(s.parse()?, Pixel::Dark))
    }
}

impl Display for Pixels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.0.height() {
            for x in 0..self.0.width() {
                write!(f, "{}", self.0.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::Cursor;

    use crate::{read_input, Input};

    const INPUT: &str = include_str!("test_input.txt");

    fn input() -> Input {
        read_input(Cursor::new(INPUT)).unwrap()
    }

    #[test]
    fn kernel() {
        let input = input();
        assert_eq!(input.1.filter_index(2, 2, Pixel::Dark), 34);
    }
}
