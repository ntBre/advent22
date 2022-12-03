use std::collections::{hash_map::RandomState, HashSet};

use advent22::*;

fn priority(b: u8) -> usize {
    if (b'A'..=b'Z').contains(&b) {
        (b - b'A' + 27) as usize
    } else {
        (b - b'a' + 1) as usize
    }
}

fn main() {
    let s = load_input();
    let mut tot = 0;
    for line in s.lines() {
        let v: Vec<u8> = line.bytes().collect();
        let (a, b) = v.split_at(v.len() / 2);
        let a: HashSet<u8, RandomState> = HashSet::from_iter(a.iter().cloned());
        let b = HashSet::from_iter(b.iter().cloned());
        let both = a.intersection(&b).next().unwrap();
        tot += priority(*both);
    }
    println!("part 1 = {tot}");
}
