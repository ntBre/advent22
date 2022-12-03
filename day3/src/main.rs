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
    let mut sets: Vec<HashSet<u8, RandomState>> = Vec::new();
    for line in s.lines() {
        sets.push(HashSet::from_iter(line.bytes()));
    }

    let mut tot = 0;
    for s in sets.chunks(3) {
        let (a, b, c) = (&s[0], &s[1], &s[2]);
        let ab: HashSet<u8, RandomState> =
            HashSet::from_iter(a.intersection(b).cloned());
        let abc = ab.intersection(c).next().unwrap();
        tot += priority(*abc);
    }
    println!("part 2 = {tot}");
}
