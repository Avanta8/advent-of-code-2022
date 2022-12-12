use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<Vec<u8>>, (usize, usize), (usize, usize));

#[aoc_generator(day12)]
fn generator(input: &str) -> Input {
    let mut grid = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = None;
    let mut end = None;
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, sq) in row.iter_mut().enumerate() {
            match *sq {
                b'S' => {
                    start = Some((i, j));
                    *sq = b'a'
                }
                b'E' => {
                    end = Some((i, j));
                    *sq = b'z'
                }
                _ => (),
            }
        }
    }

    (grid, start.unwrap(), end.unwrap())
}

#[aoc(day12, part1)]
fn solve_part1((grid, start, end): &Input) -> i32 {
    let mut bag = vec![*end];
    let mut seen = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    seen[end.0][end.1] = true;

    for count in 0.. {
        let mut new_bag = Vec::with_capacity(bag.capacity());
        for current in bag {
            if current == *start {
                return count;
            }
            let height = grid[current.0][current.1];
            for dp in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let np = (
                    (current.0 as isize + dp.0) as usize,
                    (current.1 as isize + dp.1) as usize,
                );
                if let Some(&v) = grid.get(np.0).and_then(|r| r.get(np.1)) {
                    if seen[np.0][np.1] {
                        continue;
                    }
                    if height <= v + 1 {
                        new_bag.push(np);
                        seen[np.0][np.1] = true;
                    }
                }
            }
        }
        bag = new_bag;
    }
    unreachable!()
}

#[aoc(day12, part1, impl2)]
fn solve_part1_2((grid, start, end): &Input) -> i32 {
    let mut bag = VecDeque::from([*end]);
    let mut seen = grid
        .iter()
        .map(|row| row.iter().map(|_| -1).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    seen[end.0][end.1] = 0;

    while let Some(current) = bag.pop_back() {
        let count = seen[current.0][current.1];
        if current == *start {
            return count;
        }
        let height = grid[current.0][current.1];
        for dp in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = (
                (current.0 as isize + dp.0) as usize,
                (current.1 as isize + dp.1) as usize,
            );
            if let Some(&v) = grid.get(np.0).and_then(|r| r.get(np.1)) {
                if seen[np.0][np.1] >= 0 {
                    continue;
                }
                if height <= v + 1 {
                    bag.push_front(np);
                    seen[np.0][np.1] = count + 1;
                }
            }
        }
    }
    unreachable!()
}

#[aoc(day12, part2)]
fn solve_part2((grid, _start, end): &Input) -> i32 {
    let mut bag = vec![*end];
    let mut seen = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    seen[end.0][end.1] = true;

    for count in 0.. {
        let mut new_bag = Vec::with_capacity(bag.capacity());
        for current in bag {
            let height = grid[current.0][current.1];
            if height == b'a' {
                return count;
            }
            for dp in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let np = (
                    (current.0 as isize + dp.0) as usize,
                    (current.1 as isize + dp.1) as usize,
                );
                if let Some(&v) = grid.get(np.0).and_then(|r| r.get(np.1)) {
                    if seen[np.0][np.1] {
                        continue;
                    }
                    if height <= v + 1 {
                        new_bag.push(np);
                        seen[np.0][np.1] = true;
                    }
                }
            }
        }
        bag = new_bag;
    }
    unreachable!()
}
