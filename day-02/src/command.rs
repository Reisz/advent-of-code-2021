use std::str::FromStr;

use anyhow::anyhow;

#[derive(Clone)]
pub enum Command {
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
