use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(i32, i32, i32, i32)>;

#[aoc_generator(day4)]
fn generator(input: &str) -> Input {
    input
        .split('\n')
        .map(|line| {
            let mut nums = line.split(|c| c == '-' || c == ',');
            let mut next = || nums.next().unwrap().parse().unwrap();
            (next(), next(), next(), next())
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|&&(a, b, p, q)| a <= p && b >= q || p <= a && q >= b)
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|&&(a, b, p, q)| a.max(p) <= b.min(q))
        .count()
}
