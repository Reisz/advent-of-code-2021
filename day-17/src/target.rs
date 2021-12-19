use std::{ops::RangeInclusive, str::FromStr};

use nom::Finish;

pub struct Target {
    pub x: RangeInclusive<i32>,
    pub y: RangeInclusive<i32>,
}

mod parse {
    pub use nom::error::Error;
    type Result<'a, T> = IResult<&'a str, T>;

    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, multispace0, multispace1},
        combinator::opt,
        sequence::{pair, preceded, separated_pair},
        IResult, Parser,
    };

    use super::Target;

    fn number(input: &str) -> Result<i32> {
        pair(opt(tag("-")), digit1)
            .map(|(sign, abs): (Option<&str>, &str)| if sign.is_some() { -1 } else { 1 } * abs.parse::<i32>().unwrap())
            .parse(input)
    }

    fn range(var: &'static str) -> impl FnMut(&str) -> Result<(i32, i32)> {
        move |input| {
            preceded(
                pair(tag(var), tag("=")),
                separated_pair(number, tag(".."), number),
            )(input)
        }
    }

    pub fn parse(input: &str) -> Result<Target> {
        preceded(
            pair(tag("target area:"), multispace1),
            separated_pair(range("x"), pair(tag(","), multispace0), range("y")),
        )
        .map(|((x0, x1), (y0, y1))| Target {
            x: x0..=x1,
            y: y0..=y1,
        })
        .parse(input)
    }
}

impl FromStr for Target {
    type Err = parse::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse(s)
            .finish()
            .map_err(|nom::error::Error { input, code }| Self::Err {
                input: input.to_owned(),
                code,
            })
            .map(|(_, res)| res)
    }
}
