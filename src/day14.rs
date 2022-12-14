use std::cmp::{max, min};

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

type Input = (FxHashSet<(i32, i32)>, i32);

#[aoc_generator(day14)]
fn generator(input: &str) -> Input {
    let mut grid = FxHashSet::default();
    let mut lowest = 0;
    for line in input.lines() {
        let mut coords = line.split(" -> ").map(|pair| {
            let (a, b) = pair.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        });
        let mut pos = coords.next().unwrap();
        grid.insert(pos);
        lowest = max(lowest, pos.1);
        for next in coords {
            match (pos.0 - next.0, pos.1 - next.1) {
                (0, _) => {
                    for y in min(pos.1, next.1)..=max(pos.1, next.1) {
                        grid.insert((pos.0, y));
                    }
                }
                (_, 0) => {
                    for x in min(pos.0, next.0)..=max(pos.0, next.0) {
                        grid.insert((x, pos.1));
                    }
                }
                _ => unreachable!(),
            }
            pos = next;
            lowest = max(lowest, pos.1);
        }
    }
    (grid, lowest)
}

#[aoc(day14, part1)]
fn solve_part1((grid, lowest): &Input) -> i32 {
    let mut grid = grid.clone();
    let lowest = *lowest;
    'outer: for count in 0.. {
        let mut pos = (500, 0);
        while pos.1 < lowest {
            let mut next = (pos.0, pos.1 + 1);
            if grid.contains(&next) {
                next.0 -= 1;
                if grid.contains(&next) {
                    next.0 += 2;
                    if grid.contains(&next) {
                        grid.insert(pos);
                        continue 'outer;
                    }
                }
            }
            pos = next;
        }
        return count;
    }
    unreachable!()
}

#[aoc(day14, part2)]
fn solve_part2((grid, lowest): &Input) -> i32 {
    let mut grid = grid.clone();
    let floor = *lowest + 2;
    for count in 1.. {
        let mut pos = (500, 0);
        while pos.1 + 1 < floor {
            let mut next = (pos.0, pos.1 + 1);
            if grid.contains(&next) {
                next.0 -= 1;
                if grid.contains(&next) {
                    next.0 += 2;
                    if grid.contains(&next) {
                        break;
                    }
                }
            }
            pos = next;
        }
        grid.insert(pos);
        if pos == (500, 0) {
            return count;
        }
    }
    unreachable!()
}
