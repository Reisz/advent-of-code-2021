use std::{cmp::max, io::BufRead, mem::swap};

use anyhow::Result;
use game::State;

mod game;

pub fn read_input(reader: impl BufRead) -> Result<State> {
    let mut lines = reader.lines();

    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    let (_, a) = a.split_once(':').unwrap();
    let (_, b) = b.split_once(':').unwrap();

    Ok(State::new(a.trim().parse()?, b.trim().parse()?))
}

pub fn part1(values: &State) -> usize {
    let mut state = values.clone();

    let mut die = 1;
    let mut roll_count = 0;
    let mut roll = || {
        let result = die;
        roll_count += 1;
        die = (die % 100) + 1;
        result
    };

    let loosing_score = loop {
        if let Some(mut player) = state.apply_roll(roll() + roll() + roll(), 1000) {
            player.swap();
            break state.score(player);
        }
    };

    loosing_score * roll_count
}

const SCORE_DISTRIBUTION: [(usize, usize); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub fn part2(values: &State) -> usize {
    let mut states = vec![(values.clone(), 1)];
    let mut states_buf = Vec::new();
    let mut wins = [0; 2];

    while !states.is_empty() {
        for (state, count) in states.drain(..) {
            for (roll, roll_count) in SCORE_DISTRIBUTION {
                let mut state = state.clone();
                let count = count * roll_count;

                if let Some(player) = state.apply_roll(roll, 21) {
                    wins[player.idx()] += count;
                    continue;
                }

                states_buf.push((state, count));
            }
        }

        swap(&mut states, &mut states_buf);
    }

    max(wins[0], wins[1])
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &State = &State::new(4, 8);

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT), 739785);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT), 444356092776315);
    }
}
