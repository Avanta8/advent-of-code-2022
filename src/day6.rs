use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;

fn solve(input: &str, len: usize) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    let mut set = FxHashMap::default();
    for &c in chars.iter().take(len) {
        *set.entry(c).or_insert(0) += 1;
    }
    for (i, (&a, &b)) in (len..).zip(chars.iter().zip(chars.iter().skip(len))) {
        if set.len() == len {
            return i;
        }
        *set.entry(b).or_insert(0) += 1;
        let v = set.get_mut(&a).unwrap();
        *v -= 1;
        if *v == 0 {
            set.remove(&a);
        }
    }
    input.len()
}

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    solve(input, 14)
}
