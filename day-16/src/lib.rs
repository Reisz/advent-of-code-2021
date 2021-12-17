use std::io::BufRead;

use anyhow::Result;
use packet::{Content, Operator, Packet};

mod packet;

pub fn read_input(reader: impl BufRead) -> Result<Packet> {
    Ok(reader.lines().next().unwrap()?.parse().unwrap())
}

pub fn part1(values: &Packet) -> usize {
    let mut result = 0;
    let mut todo = vec![values];

    while let Some(Packet { version, content }) = todo.pop() {
        result += *version as usize;
        if let Content::Operation(_, packets) = content {
            todo.extend(packets);
        }
    }

    result
}

fn binary<U: Into<usize>, F: Fn(&usize, &usize) -> U>(
    iter: impl Iterator<Item = usize>,
    op: F,
) -> usize {
    let [a, b]: [usize; 2] = iter.collect::<Vec<_>>().try_into().unwrap();
    op(&a, &b).into()
}

pub fn part2(values: &Packet) -> usize {
    match &values.content {
        Content::Literal(l) => *l,
        Content::Operation(op, packets) => {
            let iter = packets.iter().map(part2);
            match op {
                Operator::Sum => iter.sum(),
                Operator::Product => iter.product(),
                Operator::Min => iter.min().unwrap(),
                Operator::Max => iter.max().unwrap(),
                Operator::Gt => binary(iter, usize::gt),
                Operator::Lt => binary(iter, usize::lt),
                Operator::Eq => binary(iter, usize::eq),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &[&str] = &[
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    ];

    const INPUT2: &[&str] = &[
        "C200B40A82",
        "04005AC33890",
        "880086C3E88112",
        "CE00C43D881120",
        "D8005AC2A8F0",
        "F600BC2D8F",
        "9C005AC2F8F0",
        "9C0141080250320F1802104A08",
    ];

    fn inputs(s: &'static [&'static str]) -> impl Iterator<Item = Packet> {
        s.iter().map(|s| s.parse().unwrap())
    }

    #[test]
    fn test1() {
        for (input, value) in inputs(INPUT1).zip([16, 12, 23, 31]) {
            assert_eq!(part1(&input), value);
        }
    }

    #[test]
    fn test2() {
        for (input, value) in inputs(INPUT2).zip([3, 54, 7, 9, 1, 0, 0, 1]) {
            assert_eq!(part2(&input), value);
        }
    }
}
