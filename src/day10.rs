use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<Option<i32>>;

#[aoc_generator(day10)]
fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split_once(' ').map(|(_, b)| b.parse().unwrap()))
        .collect()
}

fn get_values(input: &Input) -> Vec<i32> {
    let mut values = vec![1];
    for &command in input {
        values.push(*values.last().unwrap());
        if let Some(value) = command {
            values.push(values.last().unwrap() + value);
        }
    }
    values
}

#[aoc(day10, part1)]
fn solve_part1(input: &Input) -> i32 {
    let values = get_values(input);
    (19..220)
        .step_by(40)
        .map(|i| (i as i32 + 1) * values[i])
        .sum()
}

#[aoc(day10, part2)]
fn solve_part2(input: &Input) -> String {
    let values = get_values(input);
    let mut screen = [['.'; 40]; 6];
    for (i, v) in values.into_iter().enumerate() {
        let p = i as i32 % 40;
        if (p - v).abs() <= 1 {
            screen[i / 40][i % 40] = '#';
        }
    }

    "\n".to_string()
        + screen
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .join("\n")
            .as_str()
}
