use std::io::BufRead;

use anyhow::Result;
use cucumber::Cucumber;
use util::grid::Grid;

mod cucumber;

type Input = Grid<Cucumber>;

pub fn read_input(mut reader: impl BufRead) -> Result<Input> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf.parse()?)
}

fn positive_wrap_once(grid: &Input, mut x: isize, mut y: isize) -> (isize, isize) {
    let w = grid.width();
    let h = grid.height();

    if x >= w {
        x -= w;
    }

    if y >= h {
        y -= h;
    }

    (x, y)
}

fn move_herd(grid: &mut Input, buf: &mut Input, herd: Cucumber) -> usize {
    buf.clear();

    let (dx, dy) = herd.next_move();
    let mut moves = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let v = *grid.get(x, y).unwrap();
            if v == Cucumber::Empty {
                continue;
            }

            let (tx, ty) = positive_wrap_once(grid, x + dx, y + dy);
            let (tx, ty) = if v == herd && *grid.get(tx, ty).unwrap() == Cucumber::Empty {
                moves += 1;
                (tx, ty)
            } else {
                (x, y)
            };

            *buf.get_mut(tx, ty).unwrap() = v;
        }
    }

    moves
}

fn step(grid: &mut Input, buf: &mut Input) -> usize {
    move_herd(grid, buf, Cucumber::East) + move_herd(buf, grid, Cucumber::South)
}

pub fn part1(values: &Grid<Cucumber>) -> usize {
    let mut grid = values.clone();
    let mut buf = values.clone();

    let mut result = 1;
    while step(&mut grid, &mut buf) > 0 {
        result += 1;
    }
    result
}

pub fn part2(_: &Grid<Cucumber>) -> usize {
    0
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
        assert_eq!(part1(&input()), 58);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 0);
    }
}
