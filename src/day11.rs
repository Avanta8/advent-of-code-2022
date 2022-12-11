use std::ops::{Add, Mul};

use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;
use regex::{Match, Regex};

type Input = (
    Vec<Vec<i64>>,
    Vec<(Box<dyn Fn(i64) -> i64>, i64, (usize, usize))>,
);

fn to_int<T>(c: Match) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    c.as_str().parse().unwrap()
}

#[aoc_generator(day11)]
fn generator(input: &str) -> Input {
    let re = Regex::new(r"\d+").unwrap();
    input
        .split("\n\n")
        .map(|para| {
            let mut parts = para.lines().skip(1);
            let mut next = || parts.next().unwrap();
            let items = {
                let captures = re.find_iter(next());
                captures.map(to_int).collect()
            };
            let operation = {
                let text = next().split_once("new =").unwrap().1;
                let split = text
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>();
                move |old| {
                    let op = if split[1] == "+" { Add::add } else { Mul::mul };
                    if split[2] == "old" {
                        op(old, old)
                    } else {
                        op(old, split[2].parse().unwrap())
                    }
                }
            };
            let test = to_int(re.find(next()).unwrap());
            let result = (
                to_int(re.find(next()).unwrap()),
                to_int(re.find(next()).unwrap()),
            );
            (
                items,
                (Box::new(operation) as Box<dyn Fn(i64) -> i64>, test, result),
            )
        })
        .unzip()
}

#[aoc(day11, part1)]
fn solve_part1((items, monkeys): &Input) -> i64 {
    let mut items = items.to_vec();
    let mut counts = vec![0; items.len()];
    for _ in 0..20 {
        for (i, (operation, test, result)) in monkeys.iter().enumerate() {
            while let Some(item) = items[i].pop() {
                counts[i] += 1;
                let worry = operation(item) / 3;
                let receiver = if worry % test == 0 {
                    result.0
                } else {
                    result.1
                };
                items[receiver].push(worry);
            }
        }
    }
    counts.sort_unstable();
    counts.reverse();
    counts[0] * counts[1]
}

#[aoc(day11, part2)]
fn solve_part2((items, monkeys): &Input) -> i64 {
    let mut items = items.to_vec();
    let mut counts = vec![0; items.len()];
    let modulo = monkeys.iter().map(|(_, t, _)| *t).reduce(lcm).unwrap();
    for _ in 0..10_000 {
        for (i, (operation, test, result)) in monkeys.iter().enumerate() {
            while let Some(item) = items[i].pop() {
                counts[i] += 1;
                let worry = operation(item) % modulo;
                let receiver = if worry % test == 0 {
                    result.0
                } else {
                    result.1
                };
                items[receiver].push(worry);
            }
        }
    }
    counts.sort_unstable();
    counts.reverse();
    counts[0] * counts[1]
}
