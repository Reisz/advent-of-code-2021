use std::io::Read;

use anyhow::Result;
use cave_system::{Cave, CaveSystem};
use itertools::Itertools;

mod cave_system;

type Input = CaveSystem;

pub fn read_input(mut reader: impl Read) -> Result<Input> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf.parse().unwrap())
}

fn find_paths<F: Fn(Cave, &[Cave]) -> bool>(system: &Input, can_revisit: F) -> Vec<Vec<Cave>> {
    let mut result = Vec::new();
    let mut todo = vec![vec![Cave::Start]];

    while let Some(partial_path) = todo.pop() {
        for next in system
            .connections(partial_path.last().unwrap())
            .unwrap()
            .iter()
            .copied()
        {
            if next == Cave::End {
                let mut path = partial_path.clone();
                path.push(next);
                result.push(path);
            } else if can_revisit(next, &partial_path) {
                let mut path = partial_path.clone();
                path.push(next);
                todo.push(path);
            }
        }
    }

    result
}

pub fn part1(values: &Input) -> usize {
    find_paths(values, |next, partial_path| {
        next.is_large() || !partial_path.contains(&next)
    })
    .len()
}

pub fn part2(values: &Input) -> usize {
    find_paths(values, |next, partial_path| {
        if next.is_large() || !partial_path.contains(&next) {
            true
        } else if next.is_small() {
            partial_path
                .iter()
                .copied()
                .filter(Cave::is_small)
                .sorted()
                .dedup_with_count()
                .all(|(count, _)| count == 1)
        } else {
            false
        }
    })
    .len()
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    const INPUTS: &[&str] = &[
        include_str!("test_input.txt"),
        include_str!("test_input_med.txt"),
        include_str!("test_input_large.txt"),
    ];

    fn inputs() -> Vec<Input> {
        INPUTS
            .iter()
            .map(|input| read_input(Cursor::new(input)).unwrap())
            .collect()
    }

    #[test]
    fn test1() {
        for (input, expect) in inputs().into_iter().zip([10, 19, 226]) {
            assert_eq!(part1(&input), expect);
        }
    }

    #[test]
    fn test2() {
        for (input, expect) in inputs().into_iter().zip([36, 103, 3509]) {
            assert_eq!(part2(&input), expect);
        }
    }
}
