use std::str::FromStr;

use anyhow::anyhow;

fn to_bits(s: &str) -> u8 {
    s.chars()
        .map(|c| c.to_digit(17).unwrap_or_else(|| panic!("{}", c)) - 10)
        .fold(0, |acc, val| acc | (1 << val))
}

pub struct SegmentDisplay(pub Vec<u8>, pub Vec<u8>);

impl FromStr for SegmentDisplay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test, values) = s
            .split_once('|')
            .ok_or(anyhow!("Could not find separator"))?;
        Ok(SegmentDisplay(
            test.trim().split_whitespace().map(to_bits).collect(),
            values.trim().split_whitespace().map(to_bits).collect(),
        ))
    }
}
