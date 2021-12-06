use anyhow::Result;

pub fn read_input<I: IntoIterator<Item = S>, S: AsRef<str>>(lines: I) -> Result<Vec<u8>> {
    lines
        .into_iter()
        .next()
        .unwrap()
        .as_ref()
        .split(',')
        .map(|n| Ok(n.parse()?))
        .collect()
}

fn calculate<'a, I: IntoIterator<Item = &'a u8>>(values: I, times: usize) -> usize {
    let mut counts = [0; 9];
    values.into_iter().for_each(|&n| counts[n as usize] += 1);
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

pub fn part1<'a, I: IntoIterator<Item = &'a u8>>(values: I) -> usize {
    calculate(values, 80)
}

pub fn part2<'a, I: IntoIterator<Item = &'a u8>>(values: I) -> usize {
    calculate(values, 256)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = &[3, 4, 3, 1, 2];

    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 5934);
        assert_eq!(part2(INPUT), 26984457539);
    }
}
