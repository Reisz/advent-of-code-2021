use std::io::Read;

use anyhow::Result;
use util::digit_grid::DigitGrid;

pub fn read_input(mut reader: impl Read) -> Result<DigitGrid> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf.parse()?)
}

fn neighbors(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
}

#[derive(Default)]
struct Flashes(usize, Vec<(isize, isize)>);

impl Flashes {
    fn inc(&mut self, val: &mut u8, x: isize, y: isize) {
        *val += 1;
        if *val > 9 {
            *val = 0;
            self.0 += 1;
            self.1.push((x, y));
        }
    }

    fn pop(&mut self) -> Option<(isize, isize)> {
        self.1.pop()
    }

    fn count(&self) -> usize {
        self.0
    }
}

fn do_step(grid: &mut DigitGrid) -> usize {
    let mut flashes = Flashes::default();

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            flashes.inc(grid.get_mut(x, y).unwrap(), x, y);
        }
    }

    while let Some((x, y)) = flashes.pop() {
        for (x, y) in neighbors(x, y) {
            if let Some(val) = grid.get_mut(x, y) {
                if *val > 0 {
                    flashes.inc(val, x, y);
                }
            }
        }
    }

    flashes.count()
}

pub fn part1(values: &DigitGrid) -> usize {
    let mut values = values.clone();
    (0..100).map(|_| do_step(&mut values)).sum()
}

pub fn part2(values: &DigitGrid) -> usize {
    let mut values = values.clone();
    #[allow(clippy::maybe_infinite_iter)]
    (1..)
        .find(|_| do_step(&mut values) == values.len())
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "5483143223\n\
                        2745854711\n\
                        5264556173\n\
                        6141336146\n\
                        6357385478\n\
                        4167524645\n\
                        2176841721\n\
                        6882881134\n\
                        4846848554\n\
                        5283751526\n";

    fn input() -> DigitGrid {
        INPUT.parse().unwrap()
    }

    #[test]
    fn test1() {
        let input = input();
        assert_eq!(part1(&input), 1656);
    }

    #[test]
    fn test2() {
        let input = input();
        assert_eq!(part2(&input), 195);
    }
}
