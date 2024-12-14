use advent_of_code_2024::day14::{part1, part2, MAP_DIMS};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day14.txt");

pub fn day14_part1_benchmark(c: &mut Criterion) {
    c.bench_function("day 14 part 1", |b| {
        b.iter(|| part1(black_box(INPUT), black_box(MAP_DIMS)))
    });
}

pub fn day14_part2_benchmark(c: &mut Criterion) {
    c.bench_function("day 14 part 2", |b| {
        b.iter(|| part2(black_box(INPUT), black_box(MAP_DIMS)))
    });
}

criterion_group!(benches, day14_part1_benchmark, day14_part2_benchmark);
criterion_main!(benches);
