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

    let tot: usize = s
        .lines()
        .map(|line| HashSet::from_iter(line.bytes()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|s| {
            let [a, b, c] = s else { unreachable!() };
            let ab: HashSet<u8, RandomState> =
                HashSet::from_iter(a.intersection(b).cloned());
            priority(*ab.intersection(c).next().unwrap())
        })
        .sum();

    assert_eq!(tot, 2838);
    println!("part 2 = {tot}");
}
