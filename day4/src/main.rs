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
        !(self.start > other.end || self.end < other.start)
    }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<_> = s.split('-').flat_map(usize::from_str).collect();
        let [start, end] = &pair[..] else { unreachable!() };
        Ok(Self {
            start: *start,
            end: *end,
        })
    }
}

fn inner(s: &str, f: fn(&Range, &Range) -> bool) -> usize {
    s.lines()
        .map(|line| {
            let rs = line.split(',').flat_map(Range::from_str);
            let [a, b] = &rs.collect::<Vec<_>>()[..] else { unreachable!() };
            (f(a, b) || f(b, a)) as usize
        })
        .sum()
}

fn main() {
    let s = load_input();
    let p1 = inner(&s, Range::contains);
    let p2 = inner(&s, Range::contains_any);

    println!("part1 = {p1}");
    println!("part2 = {p2}");
}
