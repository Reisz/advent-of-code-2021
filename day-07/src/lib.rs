use std::{cmp::min, io::BufRead};

use anyhow::{anyhow, Result};
use util::gauss_sum;

type Input = Vec<usize>;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    reader
        .lines()
        .next()
        .ok_or_else(|| anyhow!("expected input"))??
        .split(',')
        .map(|n| Ok(n.parse()?))
        .collect()
}

fn median(values: &[usize]) -> usize {
    let mut values: Vec<_> = values.iter().copied().collect();
    values.sort_unstable();
    values[values.len() / 2]
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn part1(values: &[usize]) -> usize {
    let median = median(values);
    values.iter().map(|n| abs_diff(*n, median)).sum()
}

fn avg_floor(values: &[usize]) -> usize {
    let (sum, count) = values
        .iter()
        .fold((0, 0), |(sum, count), v| (sum + v, count + 1));
    sum / count
}

pub fn part2(values: &[usize]) -> usize {
    let avg = avg_floor(values);
    let (a, b) = values
        .iter()
        .map(|n| {
            (
                gauss_sum(abs_diff(*n, avg)),
                gauss_sum(abs_diff(*n, avg + 1)),
            )
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();
    min(a, b)
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
    fn test_median() {
        assert_eq!(median(&input()), 2);
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&input()), 37);
    }

    #[test]
    fn test_avg_floor() {
        assert_eq!(avg_floor(&input()), 4);
    }

    #[test]
    fn test_gauss_sum() {
        assert_eq!(gauss_sum(1), 1);
        assert_eq!(gauss_sum(4), 10);
        assert_eq!(gauss_sum(100), 5050);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 168);
    }
}
