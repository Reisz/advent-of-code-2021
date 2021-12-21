use std::io::BufRead;

use anyhow::Result;
use pixel::{Mapping, Pixels};

mod pixel;

pub struct Input(Mapping, Pixels);

pub fn read_input(mut reader: impl BufRead) -> Result<Input> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let (mapping, grid) = buf.split_once("\n\n").unwrap();

    Ok(Input(
        mapping
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        grid.parse()?,
    ))
}

fn enhance(values: &Input, times: usize) -> usize {
    let mut grid = values.1.clone();
    for _ in 0..times {
        grid = grid.apply_filter(&values.0);
    }
    grid.count_lights()
}

pub fn part1(values: &Input) -> usize {
    enhance(values, 2)
}

pub fn part2(values: &Input) -> usize {
    enhance(values, 50)
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
        assert_eq!(part1(&input()), 35);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 3351);
    }
}
