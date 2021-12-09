use std::collections::HashSet;

use anyhow::{anyhow, Result};

pub struct Input(usize, Vec<u8>);

impl Input {
    fn at(&self, x: usize, y: usize) -> u8 {
        self.1[x + self.0 * y]
    }

    fn width(&self) -> usize {
        self.0
    }

    fn height(&self) -> usize {
        self.1.len() / self.0
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let val = self.at(x, y);

        if x > 0 && self.at(x - 1, y) <= val {
            return false;
        }
        if y > 0 && self.at(x, y - 1) <= val {
            return false;
        }
        if x + 1 < self.width() && self.at(x + 1, y) <= val {
            return false;
        }
        if y + 1 < self.height() && self.at(x, y + 1) <= val {
            return false;
        }

        true
    }
}

pub fn read_input<I: IntoIterator<Item = S>, S: AsRef<str>>(lines: I) -> Result<Input> {
    let mut width = 0;
    let values: Vec<_> = lines
        .into_iter()
        .flat_map(|l| {
            let l = l.as_ref();
            width = l.len();
            l.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or(anyhow!("Digit parsing failed."))
                        .map(|v| v.try_into().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input(width, values))
}

pub fn part1(values: &Input) -> usize {
    let mut total = 0;

    for x in 0..values.width() {
        for y in 0..values.height() {
            if values.is_low_point(x, y) {
                total += 1 + values.at(x, y) as usize;
            }
        }
    }

    total
}

pub fn part2(values: &Input) -> usize {
    let mut basins = Vec::new();

    for x in 0..values.width() {
        for y in 0..values.height() {
            if values.is_low_point(x, y) {
                let mut todo = Vec::new();
                let mut visited = HashSet::new();
                todo.push((x, y));

                while let Some((x, y)) = todo.pop() {
                    let val = values.at(x, y);
                    if val == 9 {
                        continue;
                    }

                    if x > 0 && values.at(x - 1, y) > val {
                        todo.push((x - 1, y));
                    }
                    if y > 0 && values.at(x, y - 1) > val {
                        todo.push((x, y - 1));
                    }
                    if x + 1 < values.width() && values.at(x + 1, y) > val {
                        todo.push((x + 1, y));
                    }
                    if y + 1 < values.height() && values.at(x, y + 1) > val {
                        todo.push((x, y + 1));
                    }

                    visited.insert((x, y));
                }

                basins.push(visited.len());
            }
        }
    }

    basins.sort_unstable();
    basins.reverse();
    basins[0] * basins[1] * basins[2]
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = &[
        2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
        8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
    ];

    fn input() -> Input {
        Input(10, INPUT.iter().cloned().collect())
    }

    #[test]
    fn test1() {
        let input = input();
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test2() {
        let input = input();
        assert_eq!(part2(&input), 1134);
    }
}
