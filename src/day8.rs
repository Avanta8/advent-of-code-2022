use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<i32>>;

#[aoc_generator(day8)]
fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &Input) -> usize {
    let (width, height) = (input[0].len(), input.len());
    let mut possible = vec![vec![false; width]; height];
    for (start, d1, d2, c1, c2) in [
        ((0, 0), (1, 0), (0, 1), height, width),
        ((0, 0), (0, 1), (1, 0), width, height),
        ((height - 1, width - 1), (-1, 0), (0, -1), height, width),
        ((height - 1, width - 1), (0, -1), (-1, 0), width, height),
    ] {
        for r2 in 0..c2 as isize {
            let mut tallest = -1;
            for r1 in 0..c1 as isize {
                let pos = (
                    (start.0 as isize + r1 * d1.0 + r2 * d2.0) as usize,
                    (start.1 as isize + r1 * d1.1 + r2 * d2.1) as usize,
                );
                let tree = input[pos.0][pos.1];
                if tree > tallest {
                    possible[pos.0][pos.1] = true;
                    tallest = tree;
                }
            }
        }
    }
    possible.into_iter().flatten().filter(|&x| x).count()
}

#[aoc(day8, part2)]
fn solve_part2(input: &Input) -> usize {
    let (width, height) = (input[0].len(), input.len());

    let calc_score = |i: usize, j: usize| {
        if i == 0 || j == 0 || i + 1 == height || j + 1 == width {
            return 0;
        }
        let mut score = 1;
        for (di, dj, count) in [
            (1, 0, height - i - 1),
            (0, 1, width - j - 1),
            (-1, 0, i),
            (0, -1, j),
        ] {
            let (mut pi, mut pj) = (i as isize, j as isize);
            let mut seeable = count;
            for c in 0..count {
                pi += di;
                pj += dj;
                if input[pi as usize][pj as usize] >= input[i][j] {
                    seeable = c + 1;
                    break;
                }
            }
            score *= seeable;
        }
        score
    };

    (0..height)
        .flat_map(|i| (0..width).map(move |j| calc_score(i, j)))
        .max()
        .unwrap()
}
