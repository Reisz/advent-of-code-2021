use std::{cmp::Reverse, collections::HashSet, io::Read};

use anyhow::Result;
use priority_queue::PriorityQueue;
use util::digit_grid::DigitGrid;

pub fn read_input(mut reader: impl Read) -> Result<DigitGrid> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf.parse()?)
}

fn neighbors(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)].into_iter()
}

#[derive(Debug, Eq)]
struct Node((isize, isize), usize);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<F: Fn(isize, isize) -> Option<usize>>(get_weight: F, dest: (isize, isize)) -> usize {
    let mut todo: PriorityQueue<_, _> = [((0, 0), Reverse(0))].into_iter().collect();
    let mut done = HashSet::new();

    while let Some(((x, y), weight)) = todo.pop() {
        let weight = weight.0;
        if (x, y) == dest {
            return weight;
        }

        for (x, y) in neighbors(x, y) {
            if done.contains(&(x, y)) {
                continue;
            };

            if let Some(neighbor_weight) = get_weight(x, y) {
                todo.push_increase((x, y), Reverse(weight + neighbor_weight as usize));
            }
        }

        done.insert((x, y));
    }

    panic!()
}

pub fn part1(values: &DigitGrid) -> usize {
    dijkstra(
        |x, y| values.get(x, y).map(|w| w as usize),
        (values.width() - 1, values.height() - 1),
    )
}

fn get_5_5_weight(values: &DigitGrid, x: isize, y: isize) -> Option<usize> {
    if x < 0 || y < 0 || x >= values.width() * 5 || y >= values.height() * 5 {
        return None;
    }

    values
        .get(x % values.width(), y % values.height())
        .map(|w| {
            (w as usize + (x / values.width()) as usize + (y / values.height()) as usize - 1) % 9
                + 1
        })
}

pub fn part2(values: &DigitGrid) -> usize {
    dijkstra(
        |x, y| get_5_5_weight(values, x, y),
        (values.width() * 5 - 1, values.height() * 5 - 1),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");
    const EXPANDED_INPUT: &str = include_str!("test_input_expanded.txt");

    fn input() -> DigitGrid {
        INPUT.parse().unwrap()
    }

    #[test]
    fn part2_weight() {
        let input = input();
        let expanded_input: DigitGrid = EXPANDED_INPUT.parse().unwrap();
        for x in 0..expanded_input.width() {
            for y in 0..expanded_input.height() {
                assert_eq!(
                    get_5_5_weight(&input, x, y),
                    expanded_input.get(x, y).map(|w| w as usize),
                    "{}, {}",
                    x,
                    y
                );
            }
        }

        assert_eq!(get_5_5_weight(&input, -1, 0), None);
        assert_eq!(get_5_5_weight(&input, 0, -1), None);
        assert_eq!(get_5_5_weight(&input, 50, 0), None);
        assert_eq!(get_5_5_weight(&input, 0, 50), None);
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&input()), 40);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 315);
    }
}
