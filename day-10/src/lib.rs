use std::io::BufRead;

use anyhow::Result;
use bracket::Token;

mod bracket;

pub fn read_input(reader: impl BufRead) -> Result<Vec<Vec<Token>>> {
    reader
        .lines()
        .map(|l| Ok(l?.chars().map(Token::from).collect()))
        .collect()
}

pub fn part1<'a, I: IntoIterator<Item = &'a Vec<Token>>>(values: I) -> usize {
    values
        .into_iter()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for token in line {
                match token {
                    Token::Open(bracket) => stack.push(bracket),
                    Token::Close(bracket) => {
                        if bracket != stack.pop().unwrap() {
                            return Some(bracket.checker_score());
                        }
                    }
                }
            }
            None
        })
        .sum()
}

pub fn part2<'a, I: IntoIterator<Item = &'a Vec<Token>>>(values: I) -> usize {
    let mut scores: Vec<usize> = values
        .into_iter()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for token in line {
                match token {
                    Token::Open(bracket) => stack.push(bracket),
                    Token::Close(bracket) => {
                        if bracket != stack.pop().unwrap() {
                            return None;
                        }
                    }
                }
            }

            Some(
                stack
                    .into_iter()
                    .rev()
                    .fold(0, |acc, bracket| (acc * 5) + bracket.completer_score()),
            )
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]";

    fn input() -> Vec<Vec<Token>> {
        read_input(Cursor::new(INPUT)).unwrap()
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&input()), 26397);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input()), 288957);
    }
}
