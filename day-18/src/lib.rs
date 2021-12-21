use std::{io::BufRead, ops::Add};

use anyhow::Result;
use itertools::Itertools;
use snailfish_sum::{SnailfishSum, SnailfishSums};

mod snailfish_sum;

pub fn read_input(mut reader: impl BufRead) -> Result<Vec<SnailfishSum>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf.parse::<SnailfishSums>()?.0)
}

pub fn part1<'a, I: IntoIterator<Item = &'a SnailfishSum>>(values: I) -> usize {
    values
        .into_iter()
        .cloned()
        .reduce(Add::add)
        .unwrap()
        .magnitude()
}

pub fn part2<'a, I: IntoIterator<Item = &'a SnailfishSum> + Clone>(values: I) -> usize
where
    <I as std::iter::IntoIterator>::IntoIter: std::clone::Clone,
{
    values
        .clone()
        .into_iter()
        .cartesian_product(values)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| (a.clone() + b.clone()).magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    fn input() -> Vec<SnailfishSum> {
        INPUT.parse::<SnailfishSums>().unwrap().0
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
