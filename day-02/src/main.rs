use std::str::FromStr;

use anyhow::{anyhow, Result};
use util::stdin_lines;

#[derive(Clone)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, amount) = s.split_once(' ').ok_or(anyhow!("Invalid input."))?;
        let amount = amount.parse()?;
        Ok(match name {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => return Err(anyhow!("Invalid input.")),
        })
    }
}

#[derive(Default)]
struct Position1 {
    horizontal: isize,
    vertical: isize,
}

impl Position1 {
    fn apply(mut self, command: Command) -> Self {
        match command {
            Command::Forward(amount) => self.horizontal += amount,
            Command::Down(amount) => self.vertical += amount,
            Command::Up(amount) => self.vertical -= amount,
        };
        self
    }

    fn combine(self) -> isize {
        self.horizontal * self.vertical
    }
}

#[derive(Default)]
struct Position2 {
    horizontal: isize,
    vertical: isize,
    aim: isize,
}

impl Position2 {
    fn apply(mut self, command: Command) -> Self {
        match command {
            Command::Forward(amount) => {
                self.horizontal += amount;
                self.vertical += amount * self.aim;
            }
            Command::Down(amount) => self.aim += amount,
            Command::Up(amount) => self.aim -= amount,
        };
        self
    }

    fn combine(self) -> isize {
        self.horizontal * self.vertical
    }
}

fn read_input() -> Result<Vec<Command>> {
    stdin_lines().map(|l| l.parse()).collect()
}

fn part1<I: IntoIterator<Item = Command>>(values: I) -> Position1 {
    values
        .into_iter()
        .fold(Position1::default(), Position1::apply)
}

fn part2<I: IntoIterator<Item = Command>>(values: I) -> Position2 {
    values
        .into_iter()
        .fold(Position2::default(), Position2::apply)
}

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Part 1: {}", part1(input.iter().cloned()).combine());
    println!("Part 2: {}", part2(input).combine());
    Ok(())
}
