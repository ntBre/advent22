use advent22::*;

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
    elves
}

fn main() {
    let input = load_input();
    let mut elves = part1(&input);
    elves.sort();
    elves.reverse();
    println!("part1 = {}", elves.iter().max().unwrap());
    println!("part2 = {}", elves[..3].iter().sum::<usize>());
}
