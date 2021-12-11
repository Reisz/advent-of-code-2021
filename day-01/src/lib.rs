use std::io::BufRead;

use anyhow::Result;
use itertools::Itertools;

pub fn read_input(reader: impl BufRead) -> Result<Vec<usize>> {
    reader.lines().map(|l| Ok(l?.parse()?)).collect()
}

pub fn part1<'a, I: IntoIterator<Item = &'a usize>>(values: I) -> usize {
    values
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn part2<'a, I: IntoIterator<Item = &'a usize>>(values: I) -> usize {
    part1(
        &values
            .into_iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[usize] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT), 5);
    }
}
