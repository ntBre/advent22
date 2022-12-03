use std::collections::{hash_map::RandomState, HashSet};

use advent22::*;

fn priority(b: u8) -> usize {
    if (b'A'..=b'Z').contains(&b) {
        (b - b'A' + 27) as usize
    } else {
        (b - b'a' + 1) as usize
    }
}

fn part1(s: &str) -> usize {
    s.lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let i = a.find(&b.chars().collect::<Vec<_>>()[..]).unwrap();
            priority(a.as_bytes()[i])
        })
        .sum()
}

fn part2(s: &str) -> usize {
    s.lines()
        .map(|line| HashSet::from_iter(line.bytes()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|s| {
            let [a, b, c] = s else { unreachable!() };
            let ab: HashSet<u8, RandomState> =
                HashSet::from_iter(a.intersection(b).cloned());
            priority(*ab.intersection(c).next().unwrap())
        })
        .sum()
}

fn main() {
    let s = load_input();
    let p1 = part1(&s);
    assert_eq!(p1, 7908);
    let p2 = part2(&s);
    assert_eq!(p2, 2838);
    println!("part 1 = {p1}");
    println!("part 2 = {p2}");
}
