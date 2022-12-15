use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use rustc_hash::FxHashSet;

type Input = Vec<((i32, i32), (i32, i32))>;

#[aoc_generator(day15)]
fn generator(input: &str) -> Input {
    let re = Regex::new(r"-?\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let mut maches = re.find_iter(line).map(|m| m.as_str().parse().unwrap());
            let mut next = || maches.next().unwrap();
            ((next(), next()), (next(), next()))
        })
        .collect()
}

#[aoc(day15, part1)]
fn solve_part1(input: &Input) -> usize {
    let row = 2000000;
    let mut positions = FxHashSet::default();
    for &((sx, sy), (bx, by)) in input {
        let dist = (sx - bx).abs() + (sy - by).abs();
        let offset = (row - sy).abs();
        let rem = dist - offset;
        for x in (sx - rem)..=(sx + rem) {
            positions.insert(x);
        }
    }
    for &(_, (bx, by)) in input {
        if by == row {
            positions.remove(&bx);
        }
    }
    positions.len()
}

#[aoc(day15, part2)]
fn solve_part2(input: &Input) -> i64 {
    let dim = 4000000;
    let sensors = input
        .iter()
        .map(|&((sx, sy), (bx, by))| ((sx, sy), (sx - bx).abs() + (sy - by).abs()))
        .collect::<Vec<_>>();

    for &((sx, sy), dist) in sensors.iter() {
        for (px, py) in ((sx..sx + dist).zip(sy - dist..sy))
            .chain((sx + 1..=sx + dist).rev().zip(sy..sy + dist))
            .chain((sx - dist + 1..=sx).rev().zip((sy + 1..=sy + dist).rev()))
            .chain((sx - dist..sx).zip((sy - dist + 1..=sy).rev()))
        {
            'outer: for (dx, dy) in [
                (0, 1),
                (0, -1),
                (-1, 0),
                (1, 0),
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
            ] {
                let (nx, ny) = (px + dx, py + dy);
                if nx < 0 || nx > dim || ny < 0 || ny > dim {
                    continue;
                }
                for &((tx, ty), tdist) in sensors.iter() {
                    if (tx - nx).abs() + (ty - ny).abs() <= tdist {
                        continue 'outer;
                    }
                }
                return nx as i64 * 4000000 + ny as i64;
            }
        }
    }
    unreachable!()
}
