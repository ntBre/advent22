use std::{str::FromStr, string::ParseError};

use advent22::*;

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn contains_any(&self, other: &Self) -> bool {
        if self.start > other.end {
            return false;
        }
        if self.end < other.start {
            return false;
        }
        true
    }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<_> = s.split('-').map(|s| s.parse().unwrap()).collect();
        let [start, end] = &pair[..] else {unreachable!() };
        Ok(Self {
            start: *start,
            end: *end,
        })
    }
}

fn part1(s: &str) -> usize {
    s.lines()
        .map(|line| {
            let rs = line.split(',').map(|s| Range::from_str(s).unwrap());
            let [a, b] = &rs.collect::<Vec<_>>()[..] else { todo!() };
            (a.contains(b) || b.contains(a)) as usize
        })
        .sum()
}

fn part2(s: &str) -> usize {
    s.lines()
        .map(|line| {
            let rs = line.split(',').map(|s| Range::from_str(s).unwrap());
            let [a, b] = &rs.collect::<Vec<_>>()[..] else { todo!() };
            if a.contains_any(b) || b.contains_any(a) {
                println!("{:?} {:?}", a, b);
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let s = load_input();
    let p1 = part1(&s);
    let p2 = part2(&s);

    println!("part1 = {p1}");
    println!("part2 = {p2}");
}
