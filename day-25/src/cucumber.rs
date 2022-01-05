use util::grid::Digit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cucumber {
    Empty,
    East,
    South,
}

impl Cucumber {
    pub fn next_move(self) -> (isize, isize) {
        match self {
            Cucumber::Empty => panic!(),
            Cucumber::East => (1, 0),
            Cucumber::South => (0, 1),
        }
    }
}

impl Default for Cucumber {
    fn default() -> Self {
        Self::Empty
    }
}

impl Digit for Cucumber {
    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '.' => Self::Empty,
            '>' => Self::East,
            'v' => Self::South,
            _ => return None,
        })
    }
}
