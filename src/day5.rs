use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>);

#[aoc_generator(day5)]
fn generator(input: &str) -> Input {
    let (drawing, moves) = input.split_once("\n\n").unwrap();

    let lines = drawing.lines().collect::<Vec<_>>();
    let mut formatted_drawing = vec![VecDeque::new(); 0];
    for line in &lines[..lines.len() - 1] {
        let line = line.chars().collect::<Vec<_>>();
        for (i, x) in (1..line.len()).step_by(4).enumerate() {
            if line[x] == ' ' {
                continue;
            }
            while i >= formatted_drawing.len() {
                formatted_drawing.push(VecDeque::new());
            }
            formatted_drawing[i].push_back(line[x]);
        }
    }

    let re = Regex::new(r"\d+").unwrap();
    let formatted_moves = moves
        .lines()
        .map(|line| {
            let mut captures = re.find_iter(line);
            let mut next = || captures.next().unwrap().as_str().parse::<_>().unwrap();
            (next(), next() - 1, next() - 1)
        })
        .collect();

    (formatted_drawing, formatted_moves)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> String {
    let (mut drawing, moves) = input.clone();

    for (count, from, to) in moves {
        for _ in 0..count {
            let c = drawing[from].pop_front().unwrap();
            drawing[to].push_front(c);
        }
    }

    drawing
        .into_iter()
        .map(|stack| {
            stack
                .get(0)
                .map(|c| c.to_string())
                .unwrap_or_else(|| "".to_string())
        })
        .join("")
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> String {
    let (mut drawing, moves) = input.clone();

    for (count, from, to) in moves {
        let mut taken = vec![];
        for _ in 0..count {
            taken.push(drawing[from].pop_front().unwrap())
        }
        for x in taken.into_iter().rev() {
            drawing[to].push_front(x);
        }
    }

    drawing
        .into_iter()
        .map(|stack| {
            stack
                .get(0)
                .map(|c| c.to_string())
                .unwrap_or_else(|| "".to_string())
        })
        .join("")
}
