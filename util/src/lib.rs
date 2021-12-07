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

#[macro_export]
macro_rules! main {
    ($id:ident) => {
        use anyhow::Result;
        use util::stdin_lines;
        use $id::{part1, part2, read_input};

        fn main() -> Result<()> {
            let input = read_input(stdin_lines())?;
            println!("Part1: {}", part1(&input));
            println!("Part2: {}", part2(&input));
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! bench {
    ($id:ident) => {
        use criterion::{black_box, criterion_group, criterion_main, Criterion};
        use $id::{part1, part2, read_input};

        pub fn benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group(format!("Day {:02}", &stringify!($id)[4..]));

            let lines: Vec<&str> = INPUT.lines().collect();
            group.bench_function("Parsing", |b| b.iter(|| read_input(black_box(&lines))));

            let values = read_input(lines).unwrap();
            group.bench_function("Part 1", |b| b.iter(|| part1(black_box(&values))));
            group.bench_function("Part 2", |b| b.iter(|| part2(black_box(&values))));
        }

        criterion_group!(benches, benchmark);
        criterion_main!(benches);
    };
}
