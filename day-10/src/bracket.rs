#[derive(PartialEq, Eq)]
pub enum Bracket {
    Round,
    Curly,
    Square,
    Angle,
}

impl Bracket {
    pub fn checker_score(&self) -> usize {
        match self {
            Bracket::Round => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137,
        }
    }

    pub fn completer_score(&self) -> usize {
        match self {
            Bracket::Round => 1,
            Bracket::Square => 2,
            Bracket::Curly => 3,
            Bracket::Angle => 4,
        }
    }
}

pub enum Token {
    Open(Bracket),
    Close(Bracket),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' => Token::Open(Bracket::Round),
            ')' => Token::Close(Bracket::Round),
            '{' => Token::Open(Bracket::Curly),
            '}' => Token::Close(Bracket::Curly),
            '[' => Token::Open(Bracket::Square),
            ']' => Token::Close(Bracket::Square),
            '<' => Token::Open(Bracket::Angle),
            '>' => Token::Close(Bracket::Angle),
            _ => panic!("Unknown character"),
        }
    }
}
