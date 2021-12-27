use std::io::BufRead;

use anyhow::Result;
use command::Command;
use position::Position;

mod command;
mod position;

type Input = Vec<Command>;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    reader.lines().map(|l| l?.parse()).collect()
}

pub fn part1(values: &[Command]) -> usize {
    values
        .iter()
        .fold(Position::default(), Position::apply1)
        .combine()
}

pub fn part2(values: &[Command]) -> usize {
    values
        .iter()
        .fold(Position::default(), Position::apply2)
        .combine()
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
        assert_eq!(part1(&input()), 150);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 900);
    }
}
