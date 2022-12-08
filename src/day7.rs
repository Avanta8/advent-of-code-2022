use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
enum Response {
    File(usize, String),
    Dir(String),
}

#[derive(Debug, Clone)]
enum Command {
    Cd(String),
    Ls(Vec<Response>),
}

type Input = Vec<Command>;

#[aoc_generator(day7)]
fn generator(input: &str) -> Vec<Command> {
    input
        .split("$ ")
        .filter(|x| !x.is_empty())
        .map(|text| {
            let (command, rem) = text.split_once('\n').unwrap();
            if command == "ls" {
                Command::Ls(
                    rem.lines()
                        .map(|line| {
                            let (a, b) = line.split_once(' ').unwrap();
                            if a == "dir" {
                                Response::Dir(b.to_string())
                            } else {
                                Response::File(a.parse().unwrap(), b.to_string())
                            }
                        })
                        .collect(),
                )
            } else {
                Command::Cd(command.split_once(' ').unwrap().1.to_string())
            }
        })
        .collect()
}

fn get_sizes(input: &Input) -> FxHashMap<String, usize> {
    let mut sizes = FxHashMap::default();
    let mut path = vec![];
    for command in input {
        match command {
            Command::Cd(dir) => {
                if dir == ".." {
                    path.pop();
                } else {
                    path.push(dir);
                }
            }
            Command::Ls(responses) => {
                for response in responses {
                    if let Response::File(size, _name) = response {
                        let mut current = String::new();
                        for &sup in path.iter() {
                            current.push_str(sup);
                            current.push('/');
                            *sizes.entry(current.clone()).or_insert(0) += size;
                        }
                    }
                }
            }
        }
    }
    sizes
}

#[aoc(day7, part1)]
fn solve_part1(input: &Input) -> usize {
    get_sizes(input).values().filter(|&&x| x < 100000).sum()
}

#[aoc(day7, part2)]
fn solve_part2(input: &Input) -> usize {
    let sizes = get_sizes(input);
    let req = 30000000 - (70_000_000 - sizes["//"]);
    sizes
        .values()
        .copied()
        .sorted_unstable()
        .into_iter()
        .find(|&s| s >= req)
        .unwrap()
}
