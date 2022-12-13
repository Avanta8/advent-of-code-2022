use std::{borrow::Cow, cmp::Ordering, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use once_cell::sync::OnceCell;
use regex::Regex;

static RE: OnceCell<Regex> = OnceCell::new();

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Packet(Vec<Item>);

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = RE.get_or_init(|| Regex::new(r"\[|\]|\d+").unwrap());
        let matches = re.find_iter(s).map(|m| m.as_str()).collect::<Vec<_>>();
        let mut current = vec![vec![]];
        for m in matches {
            let mut item = None;
            match m {
                "[" => current.push(vec![]),
                "]" => item = Some(Item::List(current.pop().unwrap())),
                v => item = Some(Item::Int(v.parse().unwrap())),
            }
            if let Some(item) = item {
                current.last_mut().unwrap().push(item);
            }
        }

        assert_eq!(current.len(), 1);
        Ok(Self(current.pop().unwrap()))
    }
}

#[derive(Debug, Clone)]
enum Item {
    Int(i32),
    List(Vec<Item>),
}

impl Item {
    fn as_list(&self) -> Cow<'_, Vec<Item>> {
        match self {
            Item::Int(x) => Cow::Owned(vec![Item::Int(*x)]),
            Item::List(l) => Cow::Borrowed(l),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Int(a), Item::Int(b)) => a == b,
            _ => self.as_list() == other.as_list(),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::Int(a), Item::Int(b)) => a.partial_cmp(b),
            _ => self.as_list().partial_cmp(&other.as_list()),
        }
    }
}

type Input = Vec<Packet>;

#[aoc_generator(day13)]
fn generator(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| (!line.is_empty()).then(|| line.parse().unwrap()))
        .collect()
}

#[aoc(day13, part1)]
fn solve_part1(input: &Input) -> i32 {
    (1..)
        .zip(input.chunks_exact(2))
        .filter_map(|(i, chunk)| (chunk[0] < chunk[1]).then_some(i))
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(input: &Input) -> usize {
    let p1 = Packet(vec![Item::List(vec![Item::Int(2)])]);
    let p2 = Packet(vec![Item::List(vec![Item::Int(6)])]);

    let mut input = input.to_vec();
    input.push(p1.clone());
    input.push(p2.clone());
    input.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let a = input.iter().position(|x| x == &p1).unwrap() + 1;
    let b = input.iter().position(|x| x == &p2).unwrap() + 1;
    a * b
}
