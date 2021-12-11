use std::{cmp::min, io::BufRead};

use anyhow::{anyhow, Result};

pub fn read_input(reader: impl BufRead) -> Result<Vec<usize>> {
    reader
        .lines()
        .next()
        .ok_or_else(|| anyhow!("expected input"))??
        .split(',')
        .map(|n| Ok(n.parse()?))
        .collect()
}

fn median<'a, I: IntoIterator<Item = &'a usize>>(values: I) -> usize {
    let mut values: Vec<_> = values.into_iter().copied().collect();
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

pub fn part1<'a, I: IntoIterator<Item = &'a usize> + Clone>(values: I) -> usize {
    let median = median(values.clone());
    values.into_iter().map(|n| abs_diff(*n, median)).sum()
}

fn avg_floor<'a, I: IntoIterator<Item = &'a usize>>(values: I) -> usize {
    let (sum, count) = values
        .into_iter()
        .fold((0, 0), |(sum, count), v| (sum + v, count + 1));
    sum / count
}

fn gauss_sum(n: usize) -> usize {
    (n * (n + 1)) / 2
}

pub fn part2<'a, I: IntoIterator<Item = &'a usize> + Clone>(values: I) -> usize {
    let avg = avg_floor(values.clone());
    let (a, b) = values
        .into_iter()
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
    use super::*;

    const INPUT: &[usize] = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_median() {
        assert_eq!(median(INPUT), 2);
    }

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT), 37);
    }

    #[test]
    fn test_avg_floor() {
        assert_eq!(avg_floor(INPUT), 4);
    }

    #[test]
    fn test_gauss_sum() {
        assert_eq!(gauss_sum(1), 1);
        assert_eq!(gauss_sum(4), 10);
        assert_eq!(gauss_sum(100), 5050);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT), 168);
    }
}
