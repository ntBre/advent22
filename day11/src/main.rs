#![allow(unused)]
use advent22::*;

struct Monkey {
    items: Vec<usize>,
    /// takes the old worry level and returns the new
    op: Box<dyn Fn(usize) -> usize>,
    /// performs the test and returns the monkey to throw to
    test: Box<dyn Fn(usize) -> usize>,
}

fn main() {
    let s = load_input();
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
                op: Box::new(move |old| match cur_op.as_str() {
                    "*" => old * cur_const.unwrap_or(old),
                    "+" => old + cur_const.unwrap_or(old),
                    _ => panic!("unknown op {cur_op}"),
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
                let targ = (monkeys[m].test)(worry);
                monkeys[targ].items.push(worry);
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    let p1 = inspections[0] * inspections[1];
    println!("{p1}");
}
