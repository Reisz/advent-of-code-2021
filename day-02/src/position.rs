use crate::command::Command;

#[derive(Default)]
pub struct Position {
    horizontal: isize,
    vertical: isize,
    aim: isize,
}

impl Position {
    pub fn apply1(mut self, command: &Command) -> Self {
        match command {
            Command::Forward(amount) => self.horizontal += amount,
            Command::Down(amount) => self.vertical += amount,
            Command::Up(amount) => self.vertical -= amount,
        };
        self
    }

    pub fn apply2(mut self, command: &Command) -> Self {
        match command {
            Command::Forward(amount) => {
                self.horizontal += amount;
                self.vertical += amount * self.aim;
            }
            Command::Down(amount) => self.aim += amount,
            Command::Up(amount) => self.aim -= amount,
        };
        self
    }

    pub fn combine(self) -> usize {
        (self.horizontal * self.vertical).try_into().unwrap()
    }
}
