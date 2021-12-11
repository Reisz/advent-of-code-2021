use std::io::BufRead;

use anyhow::{anyhow, Result};
use bingo::Bingo;

mod bingo;

pub struct Input(Vec<u8>, Vec<Bingo>);

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    let mut lines = reader.lines();
    let numbers = lines
        .next()
        .ok_or_else(|| anyhow!("expected input"))??
        .split(',')
        .map(|n| Ok(n.parse::<u8>()?))
        .collect::<Result<_>>()?;

    let buf: String = lines
        .map(|l| Ok(l?))
        .collect::<Result<Vec<_>>>()?
        .join("\n");
    let boards = buf
        .split("\n\n")
        .map(|b| b.parse())
        .collect::<Result<_>>()?;

    Ok(Input(numbers, boards))
}

pub fn part1(values: &Input) -> usize {
    let Input(numbers, boards) = values;
    let mut boards = boards.clone();
    for &n in numbers {
        for board in &mut boards {
            board.hit(n);
            if board.check() {
                return board.score() * n as usize;
            }
        }
    }
    unreachable!()
}

pub fn part2(values: &Input) -> usize {
    let Input(numbers, boards) = values;
    let mut boards = boards.clone();
    let mut scores = Vec::new();
    for &n in numbers {
        boards.iter_mut().for_each(|board| board.hit(n));
        boards.retain(|board| {
            let result = board.check();
            if result {
                scores.push(board.score() * n as usize);
            };
            !result
        });

        if boards.is_empty() {
            break;
        }
    }

    *scores.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_NUMBERS: &[u8] = &[
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    const INPUT_BOARDS: &[&[u8]] = &[
        &[
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ],
        &[
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6,
        ],
        &[
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ],
    ];

    fn input() -> Input {
        Input(
            INPUT_NUMBERS.iter().cloned().collect(),
            INPUT_BOARDS
                .iter()
                .cloned()
                .map(|fields| Bingo {
                    fields: fields.iter().cloned().collect(),
                    hits: 0,
                })
                .collect(),
        )
    }

    #[test]
    fn test1() {
        let input = input();
        assert_eq!(part1(&input), 4512);
    }

    #[test]
    fn test2() {
        let input = input();
        assert_eq!(part2(&input), 1924);
    }
}
