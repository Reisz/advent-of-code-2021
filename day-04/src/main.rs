use std::{
    io::{self, Read},
    str::FromStr,
};

use anyhow::Result;
use util::stdin_lines;

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
struct Bingo {
    fields: Vec<u8>,
    /// Bitmap
    hits: u32,
}

impl Bingo {
    fn hit(&mut self, val: u8) {
        for (i, field) in self.fields.iter().cloned().enumerate() {
            if val == field {
                self.hits |= 1 << i;
            }
        }
    }

    fn check(&self) -> bool {
        WINS.iter().cloned().any(|win| (self.hits & win) == win)
    }

    fn score(&self) -> usize {
        let mut score = 0;

        for (i, field) in self.fields.iter().cloned().enumerate() {
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

fn read_input() -> Result<(Vec<u8>, Vec<Bingo>)> {
    let numbers = stdin_lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| Ok(n.parse::<u8>()?))
        .collect::<Result<_>>()?;

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    let boards = buf
        .split("\n\n")
        .map(|b| b.parse())
        .collect::<Result<_>>()?;

    Ok((numbers, boards))
}

fn part1<I: IntoIterator<Item = u8>>(mut boards: Vec<Bingo>, numbers: I) -> usize {
    for n in numbers {
        for board in &mut boards {
            board.hit(n);
            if board.check() {
                return board.score() * n as usize;
            }
        }
    }
    unreachable!()
}

fn part2<I: IntoIterator<Item = u8>>(mut boards: Vec<Bingo>, numbers: I) -> usize {
    let mut scores = Vec::new();
    for n in numbers {
        boards.iter_mut().for_each(|board| board.hit(n));
        boards.retain(|board| {
            let result = board.check();
            if result {
                scores.push(board.score() * n as usize);
            };
            !result
        });

        if boards.is_empty() {
            break;
        }
    }

    *scores.last().unwrap()
}

fn main() -> Result<()> {
    let (numbers, boards) = read_input()?;
    println!("Part 1: {}", part1(boards.clone(), numbers.iter().cloned()));
    println!("Part 2: {}", part2(boards, numbers));
    Ok(())
}
