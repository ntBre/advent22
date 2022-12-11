#![allow(unused)]
use std::{
    default,
    fmt::Display,
    num::ParseIntError,
    ops::{Add, DivAssign, Mul, MulAssign, Rem},
    str::FromStr,
};

use advent22::*;

#[derive(Clone, Default)]
struct Worry {
    prod: i128,
    sum: i128,
}

impl DivAssign<i128> for Worry {
    fn div_assign(&mut self, rhs: i128) {
        self.prod = (self.prod + self.sum) / rhs;
        self.sum = 0;
    }
}

impl Rem<i128> for Worry {
    type Output = i128;

    fn rem(self, rhs: i128) -> Self::Output {
        (self.prod + self.sum) % rhs
    }
}

impl Add<i128> for Worry {
    type Output = Worry;

    fn add(mut self, rhs: i128) -> Self::Output {
        self.sum += rhs;
        self
    }
}

impl Mul<i128> for Worry {
    type Output = Worry;

    fn mul(mut self, rhs: i128) -> Self::Output {
        self.prod *= rhs;
        self
    }
}

impl From<Worry> for i128 {
    fn from(value: Worry) -> Self {
        value.prod + value.sum
    }
}

impl FromStr for Worry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse()?;
        Ok(Worry {
            prod: val,
            ..Default::default()
        })
    }
}

impl Display for Worry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.prod + self.sum)
    }
}

struct Monkey {
    items: Vec<Worry>,
    /// takes the old worry level and returns the new
    op: Box<dyn Fn(Worry) -> Worry>,
    /// performs the test and returns the monkey to throw to
    test: Box<dyn Fn(Worry) -> usize>,
}

struct Monkeys<'a>(&'a Vec<Monkey>);

impl Display for Monkeys<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, monkey) in self.0.iter().enumerate() {
            write!(f, "Monkey {i}: ")?;
            for item in &monkey.items {
                write!(f, "{item}, ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let s = load_sample();
    let mut monkeys = Vec::new();
    let mut cur_items = Vec::new();
    let mut cur_op = String::new();
    let mut cur_const = Some(0);
    let mut cur_div = 0;
    let mut true_case = 0;
    let mut false_case = 0;
    for line in s.lines() {
        if line.contains("Starting") {
            cur_items = line
                .split_ascii_whitespace()
                .skip(2)
                .map(|s| s.replace(',', "").parse().unwrap())
                .collect();
        } else if line.contains("Operation") {
            let s: Vec<_> = line.split_ascii_whitespace().skip(4).collect();
            cur_op = s[0].to_owned();
            cur_const = if let Ok(v) = s[1].parse() {
                Some(v)
            } else {
                None
            };
        } else if line.contains("Test") {
            cur_div = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        } else if line.contains("true") {
            true_case = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        } else if line.contains("false") {
            false_case = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        } else if line.is_empty() {
            let cur_op = std::mem::take(&mut cur_op);
            monkeys.push(Monkey {
                items: std::mem::take(&mut cur_items),
                op: Box::new(move |old| {
                    let c = cur_const.unwrap_or_else(|| old.clone().into());
                    match cur_op.as_str() {
                        "*" => old * c,
                        "+" => old + c,
                        _ => panic!("unknown op {cur_op}"),
                    }
                }),
                test: Box::new(move |worry| {
                    if worry % cur_div == 0 {
                        true_case
                    } else {
                        false_case
                    }
                }),
            });
        }
    }

    let mut inspections = vec![0; monkeys.len()];
    for round in 0..20 {
        for m in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[m].items);
            for mut worry in items {
                inspections[m] += 1;
                worry = (monkeys[m].op)(worry);
                worry /= 3;
                let targ = (monkeys[m].test)(worry.clone());
                monkeys[targ].items.push(worry);
            }
        }
        println!("round {}:\n{}", round + 1, Monkeys(&monkeys));
    }
    inspections.sort();
    inspections.reverse();
    dbg!(&inspections);
    let p1 = inspections[0] * inspections[1];
    println!("{p1}");
}
