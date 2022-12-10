use advent22::*;

fn main() {
    let s = load_input();

    let mut new_lines = Vec::new();
    for line in s.lines() {
        if line.starts_with("addx") {
            new_lines.push("noop");
        }
        new_lines.push(line);
    }

    let mut x: isize = 1;
    let mut tot = 0;
    let mut screen = Vec::new();
    for (cycle, line) in new_lines.iter().enumerate() {
        let px = cycle % 40;
        screen.push((x - 1..=x + 1).contains(&(px as isize)));
        let cycle = cycle as isize + 1;
        if cycle == 20 || (cycle > 40 && (cycle - 20) % 40 == 0) {
            tot += x * cycle;
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

    for row in screen.chunks(40) {
        for p in row {
            print!("{}", if *p { "#" } else { "." });
        }
        println!();
    }
    println!("{tot}");
}
