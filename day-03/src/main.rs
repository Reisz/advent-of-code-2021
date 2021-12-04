use std::{cmp::Ordering, ops::Add, str::FromStr};

use anyhow::Result;
use util::stdin_lines;

struct PowerReport {
    total_count: usize,
    ones_counts: Vec<usize>,
}

impl PowerReport {
    fn gamma_epsilon(&self) -> (usize, usize) {
        let half = self.total_count / 2;
        self.ones_counts
            .iter()
            .map(|count| count > &half)
            .fold((0, 0), |(gamma, epsilon), bit| {
                (
                    gamma << 1 | usize::from(bit),
                    epsilon << 1 | usize::from(!bit),
                )
            })
    }

    fn power_consumption(&self) -> usize {
        let (gamma, epsilon) = self.gamma_epsilon();
        gamma * epsilon
    }
}

impl FromStr for PowerReport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            total_count: 1,
            ones_counts: s.chars().map(|c| (c == '1').into()).collect(),
        })
    }
}

impl Add for PowerReport {
    type Output = PowerReport;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.total_count += rhs.total_count;
        self.ones_counts
            .iter_mut()
            .zip(rhs.ones_counts)
            .for_each(|(lhs, rhs)| *lhs += rhs);
        self
    }
}

fn read_input() -> Vec<String> {
    stdin_lines().collect()
}

fn part1<I: IntoIterator<Item = String>>(values: I) -> usize {
    values
        .into_iter()
        .map(|l| l.parse::<PowerReport>().unwrap())
        .reduce(Add::add)
        .unwrap()
        .power_consumption()
}

fn part2_counter(i: usize) -> impl Fn(&String) -> bool {
    move |v: &String| v.chars().nth(i).unwrap() == '1'
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

fn part2<I: IntoIterator<Item = String>>(values: I) -> usize {
    let (oxygen, co2): (Vec<_>, Vec<_>) = values.into_iter().partition(part2_counter(0));
    let oxygen = part2_steps(oxygen, Ordering::Greater, '1');
    let co2 = part2_steps(co2, Ordering::Less, '0');
    oxygen * co2
}

fn main() -> Result<()> {
    let input = read_input();
    println!("Part1: {}", part1(input.iter().cloned()));
    println!("Part2: {}", part2(input));
    Ok(())
}
