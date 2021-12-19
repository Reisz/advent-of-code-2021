pub mod digit_grid;

pub fn gauss_sum(n: usize) -> usize {
    (n * (n + 1)) / 2
}

#[macro_export]
macro_rules! main {
    ($id:ident) => {
        use anyhow::Result;
        use std::io;
        use $id::{part1, part2, read_input};

        fn main() -> Result<()> {
            let input = read_input(io::stdin().lock())?;
            println!("Part1: {}", part1(&input));
            println!("Part2: {}", part2(&input));
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! bench {
    ($id:ident) => {
        use std::io::Cursor;

        use criterion::{black_box, criterion_group, criterion_main, Criterion};
        use $id::{part1, part2, read_input};

        pub fn benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group(format!("Day {:02}", &stringify!($id)[4..]));

            group.bench_function("Parsing", |b| b.iter(|| read_input(Cursor::new(INPUT))));

            let values = read_input(Cursor::new(INPUT)).unwrap();
            group.bench_function("Part 1", |b| b.iter(|| part1(black_box(&values))));
            group.bench_function("Part 2", |b| b.iter(|| part2(black_box(&values))));
        }

        criterion_group!(benches, benchmark);
        criterion_main!(benches);
    };
}
