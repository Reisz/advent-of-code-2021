use std::io::BufRead;

use anyhow::Result;
use itertools::Itertools;
use line::Line;

mod line;
mod pos;

type Input = Vec<Line>;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    reader.lines().map(|l| l?.parse()).collect()
}

fn count_overlaps<'a, I: Iterator<Item = &'a Line>>(iter: I) -> usize {
    iter.flat_map(Line::points)
        .counts()
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count()
}

pub fn part1(values: &[Line]) -> usize {
    count_overlaps(values.iter().filter(|Line(a, b)| a.x == b.x || a.y == b.y))
}

pub fn part2(values: &[Line]) -> usize {
    count_overlaps(values.iter())
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
        assert_eq!(part1(&input()), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 12);
    }
}
