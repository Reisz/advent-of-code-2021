use std::io::BufRead;

use anyhow::{anyhow, Result};
use bingo::Bingo;

mod bingo;

pub struct Input(Vec<u8>, Vec<Bingo>);

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    let mut lines = reader.lines();
    let numbers = lines
        .next()
        .ok_or_else(|| anyhow!("expected input"))??
        .split(',')
        .map(|n| Ok(n.parse::<u8>()?))
        .collect::<Result<_>>()?;

    let buf: String = lines
        .map(|l| Ok(l?))
        .collect::<Result<Vec<_>>>()?
        .join("\n");
    let boards = buf.split("\n\n").map(str::parse).collect::<Result<_>>()?;

    Ok(Input(numbers, boards))
}

pub fn part1(values: &Input) -> usize {
    let Input(numbers, boards) = values;
    let mut boards = boards.clone();
    for &n in numbers {
        for board in &mut boards {
            board.hit(n);
            if board.check() {
                return board.score() * n as usize;
            }
        }
    }
    unreachable!()
}

pub fn part2(values: &Input) -> usize {
    let Input(numbers, boards) = values;
    let mut boards = boards.clone();
    let mut scores = Vec::new();
    for &n in numbers {
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

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    fn input() -> Input {
        read_input(Cursor::new(INPUT)).unwrap()
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&input()), 4512);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 1924);
    }
}
