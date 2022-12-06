use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<(i32, i32)> {
    input
        .split('\n')
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            (
                to_i32(a.chars().next().unwrap()),
                to_i32(b.chars().next().unwrap()),
            )
        })
        .collect()
}

fn to_i32(c: char) -> i32 {
    match c {
        'A' | 'X' => 0,
        'B' | 'Y' => 1,
        'C' | 'Z' => 2,
        _ => unreachable!(),
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(i32, i32)]) -> i32 {
    input
        .iter()
        .map(|&(a, b)| {
            let mut total = b + 1;
            total += if a == b {
                3
            } else if (a + 1) % 3 == b {
                6
            } else {
                0
            };
            total
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(i32, i32)]) -> i32 {
    input
        .iter()
        .map(|&(a, b)| {
            let mut total = 0;
            total += b * 3;
            total += (a + b - 1).rem_euclid(3) + 1;
            total
        })
        .sum()
}
