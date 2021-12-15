use std::{collections::HashMap, mem::swap, str::FromStr};

use itertools::{Itertools, MinMaxResult};

pub struct Rule([u8; 2], u8);

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" -> ").unwrap();
        let a = a.as_bytes().try_into().unwrap();
        let b = b.as_bytes()[0];
        Ok(Self(a, b))
    }
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pairs: HashMap<[u8; 2], usize>,
    working_copy: HashMap<[u8; 2], usize>,
    elements: HashMap<u8, usize>,
}

impl Sequence {
    fn do_step(&mut self, rules: &[Rule]) {
        self.working_copy.clear();
        for Rule(pair, insert) in rules {
            if let Some(&count) = self.pairs.get(pair) {
                *self.working_copy.entry([pair[0], *insert]).or_default() += count;
                *self.working_copy.entry([*insert, pair[1]]).or_default() += count;
                *self.elements.entry(*insert).or_default() += count;
            }
        }
        swap(&mut self.pairs, &mut self.working_copy);
    }

    fn score(&self) -> usize {
        if let MinMaxResult::MinMax(min, max) =
            self.elements.iter().map(|(_, count)| *count).minmax()
        {
            max - min
        } else {
            panic!()
        }
    }

    pub fn do_steps(&mut self, rules: &[Rule], times: usize) -> usize {
        for _ in 0..times {
            self.do_step(rules);
        }
        self.score()
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s
            .as_bytes()
            .windows(2)
            .map(|w| w.try_into().unwrap())
            .counts();
        let len = pairs.len();
        Ok(Self {
            pairs,
            working_copy: HashMap::with_capacity(len),
            elements: s.as_bytes().iter().copied().counts(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::{fmt::Debug, str::from_utf8};

    use super::*;

    fn rules() -> Vec<Rule> {
        [
            "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C", "NN -> C",
            "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N", "CN -> C",
        ]
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect()
    }

    fn seq() -> Sequence {
        "NNCB".parse::<Sequence>().unwrap()
    }

    fn cmp_sorted<T: Clone + Ord + Debug>(a: &[T], b: &[T]) {
        let mut a = a.to_vec();
        a.sort_unstable();

        let mut b = b.to_vec();
        b.sort_unstable();

        assert_eq!(a, b);
    }

    fn check(seq: Sequence, pairs: &[(&str, usize)], elements: &[(char, usize)], score: usize) {
        let actual_score = seq.score();
        cmp_sorted(
            &seq.pairs
                .iter()
                .map(|(pair, count)| (from_utf8(pair).unwrap(), *count))
                .collect::<Vec<_>>(),
            pairs,
        );
        cmp_sorted(
            &seq.elements
                .into_iter()
                .map(|(elem, count)| (elem.try_into().unwrap(), count))
                .collect::<Vec<_>>(),
            elements,
        );
        assert_eq!(actual_score, score);
    }

    #[test]
    fn before_step() {
        let seq = seq();
        check(
            seq,
            &[("CB", 1), ("NC", 1), ("NN", 1)],
            &[('B', 1), ('C', 1), ('N', 2)],
            1,
        );
    }

    #[test]
    fn one_step() {
        let pairs = &[
            ("BC", 1),
            ("CH", 1),
            ("CN", 1),
            ("HB", 1),
            ("NB", 1),
            ("NC", 1),
        ];
        let elements = &[('B', 2), ('C', 2), ('H', 1), ('N', 2)];
        let score = 1;

        let mut s = seq();
        s.do_step(&rules());
        check(s, pairs, elements, score);

        let mut s = seq();
        s.do_steps(&rules(), 1);
        check(s, pairs, elements, score);
    }

    #[test]
    fn second_step() {
        let mut seq = seq();
        seq.do_steps(&rules(), 2);
        check(
            seq,
            &[
                ("BB", 2),
                ("BC", 2),
                ("BH", 1),
                ("CB", 2),
                ("CC", 1),
                ("CN", 1),
                ("HC", 1),
                ("NB", 2),
            ],
            &[('B', 6), ('C', 4), ('H', 1), ('N', 2)],
            5,
        );
    }
}
