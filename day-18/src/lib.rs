use std::{io::BufRead, ops::Add};

use anyhow::Result;
use itertools::Itertools;
use snailfish_sum::{SnailfishSum, SnailfishSums};

mod snailfish_sum;

type Input = Vec<SnailfishSum>;

pub fn read_input(mut reader: impl BufRead) -> Result<Input> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf.parse::<SnailfishSums>()?.0)
}

pub fn part1(values: &[SnailfishSum]) -> usize {
    values.iter().cloned().reduce(Add::add).unwrap().magnitude()
}

pub fn part2(values: &[SnailfishSum]) -> usize {
    values
        .iter()
        .cartesian_product(values)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| (a.clone() + b.clone()).magnitude())
        .max()
        .unwrap()
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
        assert_eq!(part1(&input()), 4140);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 3993);
    }
}
