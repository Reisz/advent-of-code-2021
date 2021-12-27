use std::io::BufRead;

use anyhow::{anyhow, Result};

type Input = Vec<u8>;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    reader
        .lines()
        .next()
        .ok_or_else(|| anyhow!("expected input"))??
        .split(',')
        .map(|n| Ok(n.parse()?))
        .collect()
}

fn calculate(values: &[u8], times: usize) -> usize {
    let mut counts = [0; 9];
    values.iter().for_each(|&n| counts[n as usize] += 1);
    for _ in 0..times {
        counts = [
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            counts[7] + counts[0],
            counts[8],
            counts[0],
        ];
    }
    counts.into_iter().sum()
}

pub fn part1(values: &[u8]) -> usize {
    calculate(values, 80)
}

pub fn part2(values: &[u8]) -> usize {
    calculate(values, 256)
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
        assert_eq!(part1(&input()), 5934);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 26984457539);
    }
}
