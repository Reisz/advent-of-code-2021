use std::str::FromStr;

use nom::Finish;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl From<u8> for Operator {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => panic!("unexpected packet id"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Content {
    Literal(usize),
    Operation(Operator, Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub version: u8,
    pub content: Content,
}

mod parse {
    use std::str::from_utf8;

    pub use nom::error::Error;
    use nom::{
        bits::{
            bits,
            complete::{tag, take},
        },
        branch::alt,
        multi::{count, length_count, many_till},
        sequence::{pair, preceded},
        IResult, Parser,
    };

    use super::{Content, Packet};

    type Bits<'a> = (&'a [u8], usize);
    type Result<'a, T> = IResult<Bits<'a>, T>;

    fn version(input: Bits) -> Result<u8> {
        take(3_usize)(input)
    }

    fn type_id(input: Bits) -> Result<u8> {
        take(3_usize)(input)
    }

    fn literal(input: Bits) -> Result<usize> {
        many_till(
            preceded(tag(1, 1_usize), take(4_usize)),
            preceded(tag(0, 1_usize), take(4_usize)),
        )
        .map(|(list, rem): (Vec<u8>, _)| {
            let mut result = Vec::new();
            result.resize(15 - list.len(), 0);
            result.extend(list);
            result.push(rem);

            usize::from_be_bytes(
                result
                    .chunks(2)
                    .map(|chunk| (chunk[0] << 4) | chunk[1])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .parse(input)
    }

    fn literal_type_packet(input: Bits) -> Result<Content> {
        preceded(tag(4, 3_usize), literal)
            .map(Content::Literal)
            .parse(input)
    }

    fn bit_length_count<'a, F: Parser<Bits<'a>, usize, Error<Bits<'a>>>>(
        mut count_fn: F,
    ) -> impl FnMut(Bits<'a>) -> Result<Vec<Packet>> {
        move |input| {
            let (mut input, len): (_, usize) = count_fn.parse(input)?;

            // Extra code to circumvent shift overflow when taking more than 64 bits
            let cmp = {
                let times = len >> 6;
                let rem = len & 63;
                pair::<_, Vec<usize>, usize, _, _, _>(count(take(64_usize), times), take(rem))(
                    input,
                )?
                .0
            };

            let mut result = Vec::new();
            while input != cmp {
                let (new_input, value) = parse_bits(input)?;
                result.push(value);
                input = new_input;
            }
            Ok((cmp, result))
        }
    }

    fn bit_length_content(input: Bits) -> Result<Vec<Packet>> {
        preceded(tag(0, 1_usize), bit_length_count(take(15_usize)))(input)
    }

    fn count_length_content(input: Bits) -> Result<Vec<Packet>> {
        preceded(
            tag(1, 1_usize),
            length_count(take::<_, u16, _, _>(11_usize), parse_bits),
        )(input)
    }

    fn operator_type_packet(input: Bits) -> Result<Content> {
        pair(type_id, alt((bit_length_content, count_length_content)))
            .map(|(type_id, packets)| Content::Operation(type_id.into(), packets))
            .parse(input)
    }

    fn parse_bits(input: Bits) -> Result<Packet> {
        pair(version, alt((literal_type_packet, operator_type_packet)))
            .map(|(version, content)| Packet { version, content })
            .parse(input)
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Packet> {
        bits(parse_bits)(input)
    }

    pub fn to_bytes(s: &str) -> Vec<u8> {
        s.as_bytes()
            .chunks(2)
            .map(|chunk| {
                u8::from_str_radix(
                    from_utf8(&[chunk[0], chunk.get(1).copied().unwrap_or_default()]).unwrap(),
                    16,
                )
                .unwrap()
            })
            .collect()
    }
}

impl FromStr for Packet {
    type Err = parse::Error<Vec<u8>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = parse::to_bytes(s);
        parse::parse(&b)
            .finish()
            .map_err(|parse::Error { input, code }| Self::Err {
                input: input.to_vec(),
                code,
            })
            .map(|(_, res)| res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn literal_value() {
        assert_eq!(
            "D2FE28".parse::<Packet>().unwrap(),
            Packet {
                version: 6,
                content: Content::Literal(2021)
            }
        );
    }

    #[test]
    fn bit_length_operator() {
        assert_eq!(
            "38006F45291200".parse::<Packet>().unwrap(),
            Packet {
                version: 1,
                content: Content::Operation(
                    Operator::Lt,
                    vec![
                        Packet {
                            version: 6,
                            content: Content::Literal(10)
                        },
                        Packet {
                            version: 2,
                            content: Content::Literal(20)
                        }
                    ]
                )
            }
        );
    }

    #[test]
    fn count_length_operator() {
        assert_eq!(
            "EE00D40C823060".parse::<Packet>().unwrap(),
            Packet {
                version: 7,
                content: Content::Operation(
                    Operator::Max,
                    vec![
                        Packet {
                            version: 2,
                            content: Content::Literal(1)
                        },
                        Packet {
                            version: 4,
                            content: Content::Literal(2)
                        },
                        Packet {
                            version: 1,
                            content: Content::Literal(3)
                        }
                    ]
                )
            }
        );
    }
}
