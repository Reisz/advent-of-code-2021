use std::io::BufRead;

use anyhow::Result;
use target::Target;
use util::gauss_sum;

mod target;

type Input = Target;

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    Ok(reader.lines().next().unwrap().unwrap().parse()?)
}

pub fn part1(values: &Input) -> usize {
    // With `y` always `< 0` and an initial `y` velocity of `n`, the probe will rise to the gaussian
    // sum of `n`, then visit the same height levels on the way back down. Once it has reached `y =
    // 0`, its velocity will be `-n - 1`. So setting n as follows will hit the furthest vertical
    // point in the step after returning to `y = 0`.
    let initial_velocity: usize = (-(values.y.start() + 1)).try_into().unwrap();
    gauss_sum(initial_velocity)
}

pub fn part2(values: &Input) -> usize {
    let count_by_steps: Vec<usize> = vec![(values.x.end() - values.x.start()).try_into().unwrap()];
    for _ in (1..*values.x.start()).into_iter().rev() {}

    todo!("{:?}", count_by_steps)
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
        assert_eq!(part1(&input()), 45);
    }

    #[test]
    #[ignore = "unimplemented"]
    fn test2() {
        assert_eq!(part2(&input()), 112);
    }
}
