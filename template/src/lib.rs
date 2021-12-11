use std::io::BufRead;

use anyhow::Result;

pub fn read_input(reader: impl BufRead) -> Result<Vec<usize>> {
    todo!()
}

pub fn part1<'a, I: IntoIterator<Item = &'a usize>>(values: I) -> usize {
    todo!()
}

pub fn part2<'a, I: IntoIterator<Item = &'a usize>>(values: I) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[usize] = &[];

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT), 0);
    }
}
