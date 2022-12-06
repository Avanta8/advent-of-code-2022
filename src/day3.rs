use rustc_hash::FxHashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<Vec<char>> {
    input.split('\n').map(|x| x.chars().collect()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> i32 {
    input
        .iter()
        .map(|line| {
            let len = line.len();
            let first = line[..len / 2].iter().copied().collect::<FxHashSet<_>>();
            let second = line[len / 2..].iter().copied().collect::<FxHashSet<_>>();
            let item = *first.intersection(&second).next().unwrap();
            (if item.is_uppercase() {
                item as u8 - 64 + 26
            } else {
                item as u8 - 96
            }) as i32
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> i32 {
    input
        .chunks_exact(3)
        .map(|chunk| {
            let first = chunk[0].iter().collect::<FxHashSet<_>>();
            let second = chunk[1].iter().collect::<FxHashSet<_>>();
            let third = chunk[2].iter().collect::<FxHashSet<_>>();
            let item = **first
                .intersection(&second)
                .copied()
                .collect::<FxHashSet<_>>()
                .intersection(&third)
                .next()
                .unwrap();
            (if item.is_uppercase() {
                item as u8 - 64 + 26
            } else {
                item as u8 - 96
            }) as i32
        })
        .sum()
}
