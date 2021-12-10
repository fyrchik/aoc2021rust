use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day10::{part1, part2};

pub fn part1_bench(c: &mut Criterion) {
    let input = include_str!("../input");
    c.bench_function("part 1", |b| b.iter(|| part1(black_box(input))));
}

pub fn part2_bench(c: &mut Criterion) {
    let input = include_str!("../input");
    c.bench_function("part 2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, part1_bench, part2_bench);
criterion_main!(benches);
