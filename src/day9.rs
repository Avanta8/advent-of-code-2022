use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

type Input = Vec<((i32, i32), i32)>;

#[aoc_generator(day9)]
fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (d, c) = line.split_once(' ').unwrap();
            (
                match d {
                    "R" => (0, 1),
                    "U" => (-1, 0),
                    "L" => (0, -1),
                    "D" => (1, 0),
                    _ => unreachable!(),
                },
                c.parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day9, part1)]
fn solve_part1(input: &Input) -> usize {
    let mut visited = FxHashSet::default();
    visited.insert((0, 0));
    let (mut head, mut tail) = ((0, 0), (0, 0));
    for &(dp, count) in input {
        for _ in 0..count {
            head.0 += dp.0;
            head.1 += dp.1;
            let d0 = head.0 - tail.0;
            let d1 = head.1 - tail.1;
            if d0.abs() > 1 || d1.abs() > 1 {
                tail.0 += d0.signum();
                tail.1 += d1.signum();
            }
            visited.insert(tail);
        }
    }
    visited.len()
}

#[aoc(day9, part2)]
fn solve_part2(input: &Input) -> usize {
    let mut visited = FxHashSet::default();
    visited.insert((0, 0));
    let mut snake = [(0, 0); 10];
    for &(dp, count) in input {
        for _ in 0..count {
            snake[0].0 += dp.0;
            snake[0].1 += dp.1;
            for i in 0..9 {
                let head = snake[i];
                let tail = &mut snake[i + 1];
                let d0 = head.0 - tail.0;
                let d1 = head.1 - tail.1;
                if d0.abs() > 1 || d1.abs() > 1 {
                    tail.0 += d0.signum();
                    tail.1 += d1.signum();
                }
            }
            visited.insert(snake[9]);
        }
    }
    visited.len()
}
