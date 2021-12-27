use std::io::BufRead;

use anyhow::Result;

use segment_display::SegmentDisplay;
use swap_remove_pred::SwapRemovePred;

mod segment_display;
mod swap_remove_pred;

type Input = Vec<SegmentDisplay>;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    reader.lines().map(|l| l?.parse()).collect()
}

pub fn part1(values: &[SegmentDisplay]) -> usize {
    values
        .iter()
        .flat_map(|v| {
            v.1.iter().copied().map(|v| {
                let count = v.count_ones();
                (count == 2 || count == 3 || count == 4 || count == 7) as usize
            })
        })
        .sum()
}

fn find_value(input: &SegmentDisplay) -> usize {
    let mut test_values = input.0.clone();
    let mut configs = [0; 10];
    configs[1] = test_values.swap_remove_pred(|v| v.count_ones() == 2);
    configs[7] = test_values.swap_remove_pred(|v| v.count_ones() == 3);
    configs[4] = test_values.swap_remove_pred(|v| v.count_ones() == 4);
    configs[8] = test_values.swap_remove_pred(|v| v.count_ones() == 7);
    // 9 is the only remaining digits which can fully contain a 4
    configs[9] = test_values.swap_remove_pred(|v| *v & configs[4] == configs[4]);
    // 0 and 3 are the only remaining digits which can fully contain a 1
    configs[0] =
        test_values.swap_remove_pred(|v| v.count_ones() == 6 && *v & configs[1] == configs[1]);
    configs[3] =
        test_values.swap_remove_pred(|v| v.count_ones() == 5 && *v & configs[1] == configs[1]);
    // 6 is the only remaining digit with 6 segments
    configs[6] = test_values.swap_remove_pred(|v| v.count_ones() == 6);
    // 2 and 5 remain, 6 can only fully contain a 5
    configs[5] = test_values.swap_remove_pred(|v| *v & configs[6] == *v);
    configs[2] = test_values.pop().unwrap();

    input
        .1
        .iter()
        .map(|v| configs.iter().position(|c| c == v).unwrap())
        .fold(0, |acc, v| acc * 10 + v)
}

pub fn part2(values: &[SegmentDisplay]) -> usize {
    values.iter().map(find_value).sum()
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    fn input() -> Input {
        read_input(Cursor::new(INPUT)).unwrap()
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&input()), 26);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 61229);
    }
}
