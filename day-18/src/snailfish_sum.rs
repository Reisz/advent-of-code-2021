use std::{ops::Add, str::FromStr};

use nom::{error::convert_error, Finish};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnailfishSum {
    Literal(u8),
    Pair(Box<(SnailfishSum, SnailfishSum)>),
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl SnailfishSum {
    fn try_add(&mut self, value: u8, first_side: Side) -> bool {
        match self {
            SnailfishSum::Literal(n) => {
                *n += value;
                true
            }
            Self::Pair(pair) => match first_side {
                Side::Left => {
                    pair.as_mut().0.try_add(value, first_side)
                        || pair.as_mut().1.try_add(value, first_side)
                }
                Side::Right => {
                    pair.as_mut().1.try_add(value, first_side)
                        || pair.as_mut().0.try_add(value, first_side)
                }
            },
        }
    }

    fn try_explode(&mut self, depth: u8) -> Option<(Option<u8>, Option<u8>)> {
        if depth == 4 {
            if let SnailfishSum::Pair(pair) = self {
                if let (SnailfishSum::Literal(a), SnailfishSum::Literal(b)) = **pair {
                    *self = SnailfishSum::Literal(0);
                    Some((Some(a), Some(b)))
                } else {
                    panic!("Unexpected pair of pairs at depth 4")
                }
            } else {
                None
            }
        } else if let SnailfishSum::Pair(pair) = self {
            if let Some((a, b)) = pair.as_mut().0.try_explode(depth + 1) {
                Some((a, b.filter(|&val| !pair.1.try_add(val, Side::Left))))
            } else if let Some((a, b)) = pair.as_mut().1.try_explode(depth + 1) {
                Some((a.filter(|&val| !pair.0.try_add(val, Side::Right)), b))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            &mut SnailfishSum::Literal(n) => {
                if n >= 10 {
                    let half = n / 2;
                    *self = SnailfishSum::Pair(Box::new((
                        SnailfishSum::Literal(half),
                        SnailfishSum::Literal(n - half),
                    )));

                    true
                } else {
                    false
                }
            }
            SnailfishSum::Pair(pair) => {
                let (a, b) = pair.as_mut();
                a.try_split() || b.try_split()
            }
        }
    }

    fn reduce_step(&mut self) -> bool {
        self.try_explode(0).is_some() || self.try_split()
    }

    fn reduce(&mut self) {
        while self.reduce_step() {}
    }

    pub fn magnitude(&self) -> usize {
        match self {
            &SnailfishSum::Literal(n) => n as usize,
            SnailfishSum::Pair(pair) => {
                let (a, b) = pair.as_ref();
                3 * a.magnitude() + 2 * b.magnitude()
            }
        }
    }
}

impl Add for SnailfishSum {
    type Output = SnailfishSum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = SnailfishSum::Pair(Box::new((self, rhs)));
        result.reduce();
        result
    }
}

mod parse {
    pub use nom::error::VerboseError as Error;
    use nom::{
        branch::alt,
        character::complete::{char, u8},
        error::VerboseError,
        multi::separated_list1,
        sequence::{delimited, separated_pair},
        IResult, Parser,
    };

    use super::SnailfishSum;

    type Result<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

    fn literal(input: &str) -> Result<SnailfishSum> {
        u8.map(SnailfishSum::Literal).parse(input)
    }

    fn pair(input: &str) -> Result<SnailfishSum> {
        delimited(
            char('['),
            separated_pair(snailfish_sum, char(','), snailfish_sum),
            char(']'),
        )
        .map(|pair| SnailfishSum::Pair(Box::new(pair)))
        .parse(input)
    }

    pub fn snailfish_sum(input: &str) -> Result<SnailfishSum> {
        alt((literal, pair))(input)
    }

    pub fn parse(input: &str) -> Result<Vec<SnailfishSum>> {
        separated_list1(char('\n'), snailfish_sum)(input)
    }
}

pub struct SnailfishSums(pub Vec<SnailfishSum>);

impl FromStr for SnailfishSums {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse(s)
            .finish()
            .map_err(|e| anyhow::anyhow!(convert_error(s, e)))
            .map(|(_, res)| SnailfishSums(res))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse(input: &'static str) -> SnailfishSum {
        match parse::snailfish_sum(input).finish() {
            Ok((_, result)) => result,
            Err(e) => panic!("{}", convert_error(input, e)),
        }
    }

    #[test]
    fn explode() {
        let mut start = parse("[[[[[9,8],1],2],3],4]");
        assert_eq!(start.try_explode(0), Some((Some(9), None)));
        assert_eq!(start, parse("[[[[0,9],2],3],4]"));

        let mut start = parse("[7,[6,[5,[4,[3,2]]]]]");
        assert_eq!(start.try_explode(0), Some((None, Some(2))));
        assert_eq!(start, parse("[7,[6,[5,[7,0]]]]"));

        let mut start = parse("[[6,[5,[4,[3,2]]]],1]");
        assert_eq!(start.try_explode(0), Some((None, None)));
        assert_eq!(start, parse("[[6,[5,[7,0]]],3]"));

        let mut start = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert_eq!(start.try_explode(0), Some((None, None)));
        assert_eq!(start, parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

        let mut start = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(start.try_explode(0), Some((None, Some(2))));
        assert_eq!(start, parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn split() {
        let mut start = parse("[11,2]");
        assert!(start.try_split());
        assert_eq!(start, parse("[[5,6],2]"));

        let mut start = parse("[2,10]");
        assert!(start.try_split());
        assert_eq!(start, parse("[2,[5,5]]"));

        let mut start = parse("[9,9]");
        assert!(!start.try_split());
        assert_eq!(start, parse("[9,9]"));
    }

    #[test]
    fn reduce() {
        let mut start = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        start.reduce();
        assert_eq!(start, parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn add() {
        assert_eq!(
            parse("[[[[4,3],4],4],[7,[[8,4],9]]]") + parse("[1,1]"),
            parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn problematic_reduction() {
        let mut sum =
            parse("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]");

        for step in [
            "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[0,[11,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,0],[[9,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,9],[0,[11,8]]]]]",
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,9],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,0],[[7,8],5]]],[10,[[11,9],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[0,13]]],[10,[[11,9],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[0,[6,7]]]],[10,[[11,9],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[17,[[11,9],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,9],[[11,9],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,9],[[[5,6],9],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,14],[[0,15],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[0,15],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[0,[7,8]],[11,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,0],[19,0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,0],[[9,10],0]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[0,10]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[0,[5,5]]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        ] {
            assert!(sum.reduce_step());
            assert_eq!(sum, parse(step));
        }

        assert!(!sum.reduce_step());
    }

    #[test]
    fn sums() {
        assert_eq!(
            ["[1,1]", "[2,2]", "[3,3]", "[4,4]"]
                .into_iter()
                .map(parse)
                .reduce(Add::add),
            Some(parse("[[[[1,1],[2,2]],[3,3]],[4,4]]"))
        );

        assert_eq!(
            ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"]
                .into_iter()
                .map(parse)
                .reduce(Add::add),
            Some(parse("[[[[3,0],[5,3]],[4,4]],[5,5]]"))
        );

        assert_eq!(
            ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]
                .into_iter()
                .map(parse)
                .reduce(Add::add),
            Some(parse("[[[[5,0],[7,4]],[5,5]],[6,6]]"))
        );

        let mut sum = parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        for (other, expect) in [
            (
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
            (
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            ),
            (
                "[7,[5,[[3,8],[1,4]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            ),
            (
                "[[2,[2,2]],[8,[8,1]]]",
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            ),
            ("[2,9]", "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"),
            (
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            ),
            (
                "[[[5,[7,4]],7],1]",
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            ),
            (
                "[[[[4,2],2],6],[8,7]]",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ] {
            sum = sum + parse(other);
            assert_eq!(sum, parse(expect));
        }
    }
}
