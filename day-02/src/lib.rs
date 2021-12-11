use std::io::BufRead;

use anyhow::Result;
use command::Command;
use position::Position;

mod command;
mod position;

pub fn read_input(reader: impl BufRead) -> Result<Vec<Command>> {
    reader.lines().map(|l| l?.parse()).collect()
}

pub fn part1<'a, I: IntoIterator<Item = &'a Command>>(values: I) -> usize {
    values
        .into_iter()
        .fold(Position::default(), Position::apply1)
        .combine()
}

pub fn part2<'a, I: IntoIterator<Item = &'a Command>>(values: I) -> usize {
    values
        .into_iter()
        .fold(Position::default(), Position::apply2)
        .combine()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[Command] = &[
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT), 150);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT), 900);
    }
}
