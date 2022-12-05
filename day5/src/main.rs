use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;

use advent22::*;

struct Board(Vec<VecDeque<char>>);

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_length = self.0.iter().map(VecDeque::len).max().unwrap();
        for i in 0..max_length {
            for j in 0..self.0.len() {
                if let Some(x) = self.0[j].get(i) {
                    write!(f, "[{x}]")?;
                } else {
                    write!(f, "   ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let s = load_input();
    let mut in_stacks = true;
    let mut stacks = Vec::new();
    for line in s.lines() {
        if in_stacks {
            for (i, c) in line.chars().enumerate() {
                if c.is_alphabetic() {
                    let idx = (i - 1) / 4;
                    if idx >= stacks.len() {
                        stacks.resize(idx + 1, VecDeque::new());
                    }
                    stacks[idx].push_back(c);
                }
            }
        }
        if line.is_empty() {
            in_stacks = false;
            continue;
        }

        if !in_stacks {
            let sp: Vec<_> = line
                .split_ascii_whitespace()
                .flat_map(usize::from_str)
                .map(|u| u - 1)
                .collect();
            let [c, f, t] = &sp[..] else { todo!() };
            let mut tmp = Vec::new();
            for _ in 0..*c + 1 {
                let a = stacks[*f].pop_front().unwrap();
                tmp.push(a);
            }
            tmp.reverse();
            for tm in tmp {
                stacks[*t].push_front(tm);
            }
        }
    }
    let chars: String = stacks.iter().map(|s| s[0]).collect();
    println!("{}", chars);
}
