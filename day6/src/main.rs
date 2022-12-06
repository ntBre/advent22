use std::collections::HashSet;

use advent22::*;

fn main() {
    let s = load_input();
    let v: Vec<_> = s.chars().collect();
    let mut count = 14;
    for w in v.windows(14) {
        let h: HashSet<_> = w.iter().collect();
        if h.len() == 14 {
            dbg!(count);
            break;
        }
        count += 1;
    }
}
