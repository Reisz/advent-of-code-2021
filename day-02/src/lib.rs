mod command;
mod position;

use anyhow::Result;
use command::Command;
use position::Position;

pub fn read_input<I: IntoIterator<Item = S>, S: AsRef<str>>(lines: I) -> Result<Vec<Command>> {
    lines.into_iter().map(|l| l.as_ref().parse()).collect()
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
