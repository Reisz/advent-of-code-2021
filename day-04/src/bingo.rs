use std::str::FromStr;

use anyhow::Result;

#[allow(clippy::unusual_byte_groupings)]
const WINS: &[u32] = &[
    0b_11111_00000_00000_00000_00000,
    0b_00000_11111_00000_00000_00000,
    0b_00000_00000_11111_00000_00000,
    0b_00000_00000_00000_11111_00000,
    0b_00000_00000_00000_00000_11111,
    0b_10000_10000_10000_10000_10000,
    0b_01000_01000_01000_01000_01000,
    0b_00100_00100_00100_00100_00100,
    0b_00010_00010_00010_00010_00010,
    0b_00001_00001_00001_00001_00001,
];

#[derive(Debug, Clone)]
pub struct Bingo {
    pub fields: Vec<u8>,
    /// Bitmap
    pub hits: u32,
}

impl Bingo {
    pub fn hit(&mut self, val: u8) {
        for (i, field) in self.fields.iter().copied().enumerate() {
            if val == field {
                self.hits |= 1 << i;
            }
        }
    }

    pub fn check(&self) -> bool {
        WINS.iter().copied().any(|win| (self.hits & win) == win)
    }

    pub fn score(&self) -> usize {
        let mut score = 0;

        for (i, field) in self.fields.iter().copied().enumerate() {
            if self.hits & (1 << i) == 0 {
                score += field as usize;
            }
        }

        score
    }
}

impl FromStr for Bingo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            fields: s
                .split_whitespace()
                .map(|n| Ok(n.parse()?))
                .collect::<Result<_>>()?,
            hits: 0,
        })
    }
}
