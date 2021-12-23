#[derive(Debug, Clone)]
struct PlayerState {
    score: usize,
    pos: usize,
}

impl PlayerState {
    const fn new(pos: usize) -> Self {
        Self { score: 0, pos }
    }

    fn apply_roll(&mut self, roll: usize) -> usize {
        self.pos = ((self.pos + roll - 1) % 10) + 1;
        self.score += self.pos;
        self.score
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn idx(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
        }
    }

    pub fn swap(&mut self) {
        *self = match self {
            Self::A => Self::B,
            Self::B => Self::A,
        };
    }
}

#[derive(Debug, Clone)]
pub struct State {
    players: [PlayerState; 2],
    current_player: Player,
}

impl State {
    pub const fn new(a_pos: usize, b_pos: usize) -> Self {
        Self {
            players: [PlayerState::new(a_pos), PlayerState::new(b_pos)],
            current_player: Player::A,
        }
    }

    pub fn apply_roll(&mut self, roll: usize, winning_score: usize) -> Option<Player> {
        let score = self.players[self.current_player.idx()].apply_roll(roll);
        let result = (score >= winning_score).then(|| self.current_player);
        self.current_player.swap();
        result
    }

    pub fn score(&self, player: Player) -> usize {
        self.players[player.idx()].score
    }
}
