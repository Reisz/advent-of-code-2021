use std::io::BufRead;

use anyhow::Result;
use sequence::{Rule, Sequence};

mod sequence;

pub struct Input(Sequence, Vec<Rule>);

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    let mut lines = reader.lines();
    let sequence = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(lines.next().unwrap().unwrap().is_empty());

    Ok(Input(
        sequence,
        lines.map(|l| l.unwrap().parse().unwrap()).collect(),
    ))
}

pub fn part1(values: &Input) -> usize {
    values.0.clone().do_steps(&values.1, 10)
}

pub fn part2(values: &Input) -> usize {
    values.0.clone().do_steps(&values.1, 40)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    const INPUT: &str = "NNCB\n\
                        \n\
                        CH -> B\n\
                        HH -> N\n\
                        CB -> H\n\
                        NH -> C\n\
                        HB -> C\n\
                        HC -> B\n\
                        HN -> C\n\
                        NN -> C\n\
                        BH -> H\n\
                        NC -> B\n\
                        NB -> B\n\
                        BN -> B\n\
                        BB -> N\n\
                        BC -> B\n\
                        CC -> N\n\
                        CN -> C";

    fn input() -> Input {
        read_input(Cursor::new(INPUT)).unwrap()
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&input()), 1588);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 2188189693529);
    }
}
