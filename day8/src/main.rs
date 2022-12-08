use advent22::*;

fn part1(grid: &Vec<Vec<u32>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut tot = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if i == 0
                || i == rows - 1
                || j == 0
                || j == cols - 1
                || grid.iter().take(i).all(|x| *col > x[j])
                || grid.iter().rev().take(rows - i - 1).all(|x| *col > x[j])
                || grid[i].iter().take(j).all(|x| col > x)
                || grid[i].iter().rev().take(cols - j - 1).all(|x| col > x)
            {
                tot += 1;
                continue;
            }
        }
    }
    tot
}

fn part2(grid: Vec<Vec<u32>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut scores = vec![vec![1; cols]; rows];
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if i == 0 || i == rows - 1 {
                scores[i][j] = 0;
                continue;
            }
            if j == 0 || j == cols - 1 {
                scores[i][j] = 0;
                continue;
            }

            let mut count = 0;
            for ii in (0..i).rev() {
                count += 1;
                if *col <= grid[ii][j] {
                    break;
                }
            }
            scores[i][j] *= count;
            count = 0;
            for ii in i + 1..rows {
                count += 1;
                if *col <= grid[ii][j] {
                    break;
                }
            }
            scores[i][j] *= count;
            count = 0;

            for jj in (0..j).rev() {
                count += 1;
                if *col <= grid[i][jj] {
                    break;
                }
            }
            scores[i][j] *= count;
            count = 0;

            for jj in j + 1..cols {
                count += 1;
                if *col <= grid[i][jj] {
                    break;
                }
            }
            scores[i][j] *= count;
        }
    }
    *scores
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
}

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

    let p1 = part1(&grid);
    let p2 = part2(grid);
    assert_eq!(p1, 1843);
    assert_eq!(p2, 180000);
    println!("{}", p1);
    println!("{}", p2);
}
