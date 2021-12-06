use std::{ops::Add, str::FromStr};

pub struct PowerReport {
    total_count: usize,
    ones_counts: Vec<usize>,
}

impl PowerReport {
    pub fn gamma_epsilon(&self) -> (usize, usize) {
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

    pub fn power_consumption(&self) -> usize {
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
