mod power_report;

use std::{cmp::Ordering, ops::Add};

use anyhow::Result;
use power_report::PowerReport;

pub fn read_input<I: IntoIterator<Item = S>, S: AsRef<str>>(lines: I) -> Result<Vec<String>> {
    Ok(lines.into_iter().map(|s| s.as_ref().to_owned()).collect())
}

pub fn part1<I: IntoIterator<Item = S>, S: AsRef<str>>(values: I) -> usize {
    values
        .into_iter()
        .map(|l| l.as_ref().parse::<PowerReport>().unwrap())
        .reduce(Add::add)
        .unwrap()
        .power_consumption()
}

fn part2_counter(i: usize) -> impl Fn(&String) -> bool {
    move |v| v.chars().nth(i).unwrap() == '1'
}

fn part2_steps(mut input: Vec<String>, req_ordering: Ordering, decider: char) -> usize {
    let mut idx = 1;
    while input.len() > 1 {
        let (a, b): (Vec<_>, Vec<_>) = input.into_iter().partition(part2_counter(idx));
        let ord = a.len().cmp(&b.len());
        input = ((decider == '1' && ord.is_eq()) || ord == req_ordering)
            .then(move || a)
            .unwrap_or(b);
        idx += 1;
    }
    usize::from_str_radix(&input[0], 2).unwrap()
}

pub fn part2<I: IntoIterator<Item = S>, S: AsRef<str>>(values: I) -> usize {
    let (oxygen, co2): (Vec<_>, Vec<_>) = values
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .partition(part2_counter(0));
    let oxygen = part2_steps(oxygen, Ordering::Greater, '1');
    let co2 = part2_steps(co2, Ordering::Less, '0');
    oxygen * co2
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 198);
        assert_eq!(part2(INPUT), 230);
    }
}
