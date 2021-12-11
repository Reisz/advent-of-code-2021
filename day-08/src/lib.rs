mod swap_remove_pred;

use std::{io::BufRead, str::FromStr};

use anyhow::{anyhow, Result};

use swap_remove_pred::SwapRemovePred;

fn to_bits(s: &str) -> u8 {
    s.chars()
        .map(|c| c.to_digit(17).unwrap_or_else(|| panic!("{}", c)) - 10)
        .fold(0, |acc, val| acc | (1 << val))
}

pub struct Input(pub Vec<u8>, pub Vec<u8>);

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test, values) = s
            .split_once('|')
            .ok_or(anyhow!("Could not find separator"))?;
        Ok(Input(
            test.trim().split_whitespace().map(to_bits).collect(),
            values.trim().split_whitespace().map(to_bits).collect(),
        ))
    }
}

pub fn read_input(reader: impl BufRead) -> Result<Vec<Input>> {
    reader.lines().map(|l| l?.parse()).collect()
}

pub fn part1<'a, I: IntoIterator<Item = &'a Input>>(values: I) -> usize {
    values
        .into_iter()
        .flat_map(|v| {
            v.1.iter().copied().map(|v| {
                let count = v.count_ones();
                (count == 2 || count == 3 || count == 4 || count == 7) as usize
            })
        })
        .sum()
}

fn find_value(input: &Input) -> usize {
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

pub fn part2<'a, I: IntoIterator<Item = &'a Input>>(values: I) -> usize {
    values.into_iter().map(find_value).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[([&str; 10], [&str; 4])] = &[
        (
            [
                "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd",
                "edb",
            ],
            ["fdgacbe", "cefdb", "cefbgd", "gcbe"],
        ),
        (
            [
                "edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde", "gfcbed",
                "gfec",
            ],
            ["fcgedb", "cgb", "dgebacf", "gc"],
        ),
        (
            [
                "fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb",
                "cdgabef",
            ],
            ["cg", "cg", "fdcagb", "cbg"],
        ),
        (
            [
                "fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca",
                "fcdbega",
            ],
            ["efabcd", "cedba", "gadfec", "cb"],
        ),
        (
            [
                "aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab",
                "fcbdga",
            ],
            ["gecf", "egdcabf", "bgf", "bfgea"],
        ),
        (
            [
                "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg", "bafgc",
                "acf",
            ],
            ["gebdcfa", "ecba", "ca", "fadegcb"],
        ),
        (
            [
                "dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb", "gdcebf",
                "gf",
            ],
            ["cefg", "dcbef", "fcge", "gbcadfe"],
        ),
        (
            [
                "bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg",
                "gebcd",
            ],
            ["ed", "bcgafe", "cdgba", "cbgef"],
        ),
        (
            [
                "egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb",
                "bfceg",
            ],
            ["gbdfcae", "bgc", "cg", "cgb"],
        ),
        (
            [
                "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac",
                "fegbdc",
            ],
            ["fgae", "cfgab", "fg", "bagce"],
        ),
    ];

    fn input() -> Vec<Input> {
        INPUT
            .iter()
            .map(|i| {
                Input(
                    i.0.iter().cloned().map(to_bits).collect(),
                    i.1.iter().cloned().map(to_bits).collect(),
                )
            })
            .collect()
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
