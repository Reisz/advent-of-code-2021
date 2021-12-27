use std::io::BufRead;

use anyhow::Result;
use itertools::Itertools;

type Input = Vec<usize>;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    reader.lines().map(|l| Ok(l?.parse()?)).collect()
}

pub fn part1(values: &[usize]) -> usize {
    values.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

pub fn part2(values: &[usize]) -> usize {
    part1(
        &values
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect::<Vec<_>>(),
    )
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
        assert_eq!(part1(&input()), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 5);
    }
}
