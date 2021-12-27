use std::io::BufRead;

use anyhow::Result;

type Input = Vec<usize>;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    todo!()
}

pub fn part1(values: &[usize]) -> usize {
    todo!()
}

pub fn part2(values: &[usize]) -> usize {
    todo!()
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
        assert_eq!(part1(&input()), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 0);
    }
}
