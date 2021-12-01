use anyhow::Result;
use itertools::Itertools;

use std::io;

fn read_input() -> Result<Vec<usize>> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut result = Vec::new();
    loop {
        stdin.read_line(&mut buffer)?;
        buffer.pop();

        if buffer.is_empty() {
            break;
        }

        result.push(buffer.parse()?);
        buffer.clear();
    }
    Ok(result)
}

fn process<I: IntoIterator<Item = usize>>(values: I) -> usize {
    values
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

fn main() -> Result<()> {
    println!("{}", process(read_input()?));
    Ok(())
}
