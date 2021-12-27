use std::{cmp::Ordering, io::BufRead, ops::Add};

use anyhow::Result;
use power_report::PowerReport;

mod power_report;

type Input = Vec<String>;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    reader.lines().map(|l| Ok(l?)).collect()
}

pub fn part1(values: &[String]) -> usize {
    values
        .iter()
        .map(|l| l.parse::<PowerReport>().unwrap())
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

pub fn part2(values: &[String]) -> usize {
    let (oxygen, co2): (Vec<_>, Vec<_>) = values.iter().cloned().partition(part2_counter(0));
    let oxygen = part2_steps(oxygen, Ordering::Greater, '1');
    let co2 = part2_steps(co2, Ordering::Less, '0');
    oxygen * co2
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
        assert_eq!(part1(&input()), 198);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 230);
    }
}
