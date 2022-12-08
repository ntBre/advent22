use advent22::*;

fn main() {
    let s = load_input();
    let grid: Vec<_> = s
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| char::to_digit(c, 10))
                .collect::<Vec<_>>()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut tot = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if i == 0 || i == rows - 1 {
                tot += 1;
                continue;
            }
            if j == 0 || j == cols - 1 {
                tot += 1;
                continue;
            }

            let mut found = true;
            for ii in 0..i {
                if *col <= grid[ii][j] {
                    found = false;
                    break;
                }
            }

            if found {
                tot += 1;
                continue;
            }

            found = true;
            for ii in i + 1..rows {
                if *col <= grid[ii][j] {
                    found = false;
                    break;
                }
            }
            if found {
                tot += 1;
                continue;
            }

            found = true;
            for jj in 0..j {
                if *col <= grid[i][jj] {
                    found = false;
                    break;
                }
            }

            if found {
                tot += 1;
                continue;
            }

            found = true;
            for jj in j + 1..cols {
                if *col <= grid[i][jj] {
                    found = false;
                    break;
                }
            }

            if found {
                tot += 1;
                continue;
            }
        }
    }
    println!("{tot}");
}
