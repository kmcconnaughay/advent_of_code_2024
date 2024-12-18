use advent_of_code_2024::day18::{part1, part2, MAP_DIMS};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day18.txt");

pub fn day18_part1_benchmark(c: &mut Criterion) {
    c.bench_function("day 18 part 1", |b| {
        b.iter(|| part1(black_box(INPUT), black_box(MAP_DIMS), black_box(1024)))
    });
}

pub fn day18_part2_benchmark(c: &mut Criterion) {
    c.bench_function("day 18 part 2", |b| {
        b.iter(|| part2(black_box(INPUT), black_box(MAP_DIMS)))
    });
}

criterion_group!(benches, day18_part1_benchmark, day18_part2_benchmark);
criterion_main!(benches);
