mod line;
mod pos;

use anyhow::Result;
use itertools::Itertools;
use line::Line;

pub fn read_input<I: IntoIterator<Item = S>, S: AsRef<str>>(lines: I) -> Result<Vec<Line>> {
    lines.into_iter().map(|l| l.as_ref().parse()).collect()
}

pub fn part1<'a, I: IntoIterator<Item = &'a Line>>(values: I) -> usize {
    part2(
        values
            .into_iter()
            .filter(|Line(a, b)| a.x == b.x || a.y == b.y),
    )
}

pub fn part2<'a, I: IntoIterator<Item = &'a Line>>(values: I) -> usize {
    values
        .into_iter()
        .flat_map(|l| l.points())
        .counts()
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pos::Pos;

    const INPUT: &[Line] = &[
        Line(Pos { x: 0, y: 9 }, Pos { x: 5, y: 9 }),
        Line(Pos { x: 8, y: 0 }, Pos { x: 0, y: 8 }),
        Line(Pos { x: 9, y: 4 }, Pos { x: 3, y: 4 }),
        Line(Pos { x: 2, y: 2 }, Pos { x: 2, y: 1 }),
        Line(Pos { x: 7, y: 0 }, Pos { x: 7, y: 4 }),
        Line(Pos { x: 6, y: 4 }, Pos { x: 2, y: 0 }),
        Line(Pos { x: 0, y: 9 }, Pos { x: 2, y: 9 }),
        Line(Pos { x: 3, y: 4 }, Pos { x: 1, y: 4 }),
        Line(Pos { x: 0, y: 0 }, Pos { x: 8, y: 8 }),
        Line(Pos { x: 5, y: 5 }, Pos { x: 8, y: 2 }),
    ];

    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 5);
        assert_eq!(part2(INPUT), 12);
    }
}
