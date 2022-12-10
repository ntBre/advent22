use advent22::*;

#[allow(unused)]
fn main() {
    let s = load_input();

    let mut new_lines = Vec::new();
    for line in s.lines() {
        if line.starts_with("addx") {
            new_lines.push("noop");
        }
        new_lines.push(line);
    }

    let mut x = 1;
    let mut tot = 0;
    let mut cycle = 1;
    for (i, line) in new_lines.iter().enumerate() {
        let i = i as isize + 1;
        if i == 20 || (i > 40 && (i - 20) % 40 == 0) {
            tot += x * i;
        }
        if line.starts_with("addx") {
            let v: isize = line
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            x += v;
        }
    }
    println!("{tot}");
}
