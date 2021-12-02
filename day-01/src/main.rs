use anyhow::Result;
use itertools::Itertools;
use util::stdin_lines;

fn read_input() -> Result<Vec<usize>> {
    stdin_lines().map(|l| Ok(l.parse()?)).collect()
}

fn part1<I: IntoIterator<Item = usize>>(values: I) -> usize {
    values
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

fn part2<I: IntoIterator<Item = usize>>(values: I) -> usize {
    part1(
        values
            .into_iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c),
    )
}

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Part 1: {}", part1(input.iter().cloned()));
    println!("Part 2: {}", part2(input));
    Ok(())
}
