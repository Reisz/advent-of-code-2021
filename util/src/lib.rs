use std::io;

pub struct StdinLines {
    stdin: io::Stdin,
}

impl Iterator for StdinLines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        self.stdin
            .read_line(&mut buffer)
            .expect("Could not read line.");
        buffer.pop();

        Some(buffer).filter(|l| !l.is_empty())
    }
}

pub fn stdin_lines() -> StdinLines {
    StdinLines { stdin: io::stdin() }
}
