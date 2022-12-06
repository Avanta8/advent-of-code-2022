use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generator(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n\n")
        .map(|group| group.split('\n').map(|x| x.parse().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|group| group.iter().sum::<i64>())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<i64>]) -> i64 {
    let mut arr = input
        .iter()
        .map(|group| group.iter().sum::<i64>())
        .collect::<Vec<_>>();
    arr.sort();
    arr.reverse();
    arr[..3].iter().sum()
}
