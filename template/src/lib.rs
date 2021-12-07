use anyhow::Result;

pub fn read_input<I: IntoIterator<Item = S>, S: AsRef<str>>(lines: I) -> Result<Vec<usize>> {
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
    fn test() {
        assert_eq!(part1(INPUT), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
