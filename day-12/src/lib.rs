use std::io::Read;

use anyhow::Result;
use cave_system::{Cave, CaveSystem};
use itertools::Itertools;

mod cave_system;

pub fn read_input(mut reader: impl Read) -> Result<CaveSystem> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf.parse().unwrap())
}

fn find_paths<F: Fn(Cave, &[Cave]) -> bool>(system: &CaveSystem, can_revisit: F) -> Vec<Vec<Cave>> {
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

pub fn part1(values: &CaveSystem) -> usize {
    find_paths(values, |next, partial_path| {
        next.is_large() || !partial_path.contains(&next)
    })
    .len()
}

pub fn part2(values: &CaveSystem) -> usize {
    find_paths(values, |next, partial_path| {
        if next.is_large() || !partial_path.contains(&next) {
            true
        } else if next.is_small() {
            partial_path
                .iter()
                .copied()
                .filter(Cave::is_small)
                .counts()
                .iter()
                .all(|(_, count)| *count == 1)
        } else {
            false
        }
    })
    .len()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "start-A\n\
                        start-b\n\
                        A-c\n\
                        A-b\n\
                        b-d\n\
                        A-end\n\
                        b-end";

    const INPUT2: &str = "dc-end\n\
                        HN-start\n\
                        start-kj\n\
                        dc-start\n\
                        dc-HN\n\
                        LN-dc\n\
                        HN-end\n\
                        kj-sa\n\
                        kj-HN\n\
                        kj-dc";

    const INPUT3: &str = "fs-end\n\
                        he-DX\n\
                        fs-he\n\
                        start-DX\n\
                        pj-DX\n\
                        end-zg\n\
                        zg-sl\n\
                        zg-pj\n\
                        pj-he\n\
                        RW-he\n\
                        fs-DX\n\
                        pj-RW\n\
                        zg-RW\n\
                        start-pj\n\
                        he-WI\n\
                        zg-he\n\
                        pj-fs\n\
                        start-RW";

    fn input1() -> CaveSystem {
        INPUT1.parse().unwrap()
    }

    fn input2() -> CaveSystem {
        INPUT2.parse().unwrap()
    }

    fn input3() -> CaveSystem {
        INPUT3.parse().unwrap()
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&input1()), 10);
        assert_eq!(part1(&input2()), 19);
        assert_eq!(part1(&input3()), 226);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input1()), 36);
        assert_eq!(part2(&input2()), 103);
        assert_eq!(part2(&input3()), 3509);
    }
}
