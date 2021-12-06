use anyhow::Result;
use util::stdin_lines;

fn calculate<I: IntoIterator<Item = u8>>(values: I, times: usize) -> usize {
    let mut counts = [0; 9];
    values.into_iter().for_each(|n| counts[n as usize] += 1);
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

fn read_input() -> Result<Vec<u8>> {
    stdin_lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| Ok(n.parse()?))
        .collect()
}

fn main() -> Result<()> {
    let values = read_input()?;
    println!("Part 1: {}", calculate(values.iter().cloned(), 80));
    println!("Part 2: {}", calculate(values, 256));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = &[3, 4, 3, 1, 2];

    #[test]
    fn test() {
        assert_eq!(calculate(INPUT.iter().cloned(), 80), 5934);
        assert_eq!(calculate(INPUT.iter().cloned(), 256), 26984457539);
    }
}
