use advent22::*;

fn main() {
    let input = load_input();
    let mut elves = part1(&input);
    elves.sort();
    elves.reverse();
    dbg!(elves[..3].iter().sum::<usize>());
    // println!("{}", input);
}

fn part1(inp: &str) -> Vec<usize> {
    let mut elves = Vec::new();
    let mut sum = 0;
    for line in inp.lines() {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<usize>().unwrap();
        }
    }
    elves.push(sum);
    println!("max = {}", elves.iter().max().unwrap());
    elves
}
