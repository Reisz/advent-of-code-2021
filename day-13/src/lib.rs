use std::{cmp::max, io::BufRead};

use anyhow::Result;
use paper::{Fold, Pos};

mod paper;

#[derive(Clone)]
pub struct Input {
    dots: Vec<Pos>,
    folds: Vec<Fold>,
}

pub fn read_input(reader: impl BufRead) -> Result<Input> {
    let mut lines = reader.lines();

    let mut dots = Vec::new();
    while let Some(Ok(l)) = lines.next() {
        if l.is_empty() {
            break;
        }

        dots.push(l.parse()?);
    }

    let mut folds = Vec::new();
    while let Some(Ok(l)) = lines.next() {
        folds.push(l.parse()?);
    }

    Ok(Input { dots, folds })
}

fn do_fold(fold: &Fold, dots: &mut Vec<Pos>) {
    dots.iter_mut().for_each(|dot| *dot = fold.apply_to(*dot));
    dots.sort_unstable();
    dots.dedup();
}

pub fn part1(values: &Input) -> usize {
    let mut dots = values.dots.clone();
    do_fold(values.folds.first().unwrap(), &mut dots);
    dots.len()
}

pub fn part2(values: &Input) -> usize {
    let mut dots = values.dots.clone();
    values
        .folds
        .iter()
        .for_each(|fold| do_fold(fold, &mut dots));

    let (w, h) = dots
        .iter()
        .fold((0, 0), |(w, h), Pos(x, y)| (max(w, *x), max(h, *y)));

    for y in 0..=h {
        for x in 0..=w {
            print!(
                "{}",
                if dots.binary_search(&Pos(x, y)).is_ok() {
                    '#'
                } else {
                    ' '
                }
            );
        }

        println!();
    }

    dots.len()
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
        assert_eq!(part1(&input()), 17);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 16);
    }
}
