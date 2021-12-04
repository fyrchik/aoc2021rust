use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day4::*;
use std::fs;

fn get_input(filename: &str) -> Result<(Vec<usize>, Vec<Table>)> {
    let input = fs::read_to_string(filename)?;
    parse(&input)
}

pub fn part1_bench(c: &mut Criterion) {
    let (numbers, tables) = get_input("input").unwrap();
    c.bench_function("part1", |b| {
        b.iter(|| part1(black_box(&numbers), black_box(&tables)));
    });
}

pub fn part2_bench(c: &mut Criterion) {
    let (numbers, tables) = get_input("input").unwrap();
    c.bench_function("part2", |b| {
        b.iter(|| part2(black_box(&numbers), black_box(&tables)))
    });
}

criterion_group!(benches, part1_bench, part2_bench);
criterion_main!(benches);
