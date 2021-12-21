use std::{collections::HashSet, io::Read};

use anyhow::Result;
use util::grid::Grid;

pub fn read_input(mut reader: impl Read) -> Result<Grid<u8>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf.parse()?)
}

fn neighbors(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)].into_iter()
}

fn is_low_point(grid: &Grid<u8>, x: isize, y: isize) -> bool {
    let val = grid.get(x, y).unwrap();
    for (x, y) in neighbors(x, y) {
        if grid.get(x, y).map_or(false, |neighbor| val >= neighbor) {
            return false;
        }
    }
    true
}

pub fn part1(values: &Grid<u8>) -> usize {
    let mut total = 0;

    for x in 0..values.width() {
        for y in 0..values.height() {
            if is_low_point(values, x, y) {
                total += 1 + *values.get(x, y).unwrap() as usize;
            }
        }
    }

    total
}

pub fn part2(values: &Grid<u8>) -> usize {
    let mut basins: Vec<usize> = Vec::new();

    for x in 0..values.width() {
        for y in 0..values.height() {
            if is_low_point(values, x, y) {
                let mut todo = Vec::new();
                let mut visited = HashSet::new();
                todo.push((x, y));

                while let Some((x, y)) = todo.pop() {
                    let val = *values.get(x, y).unwrap();
                    if val == 9 {
                        continue;
                    }

                    for (x, y) in neighbors(x, y) {
                        if values.get(x, y).map_or(false, |neighbor| *neighbor > val) {
                            todo.push((x, y));
                        }
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

    const INPUT: &str = "2199943210\n\
                        3987894921\n\
                        9856789892\n\
                        8767896789\n\
                        9899965678\n";

    fn input() -> Grid<u8> {
        INPUT.parse().unwrap()
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
