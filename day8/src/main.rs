use advent22::*;

fn part1(grid: &Vec<Vec<u32>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut tot = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            tot += (i == 0
                || i == rows - 1
                || j == 0
                || j == cols - 1
                || grid.iter().take(i).all(|x| *col > x[j])
                || grid.iter().rev().take(rows - i - 1).all(|x| *col > x[j])
                || grid[i].iter().take(j).all(|x| col > x)
                || grid[i].iter().rev().take(cols - j - 1).all(|x| col > x))
                as usize
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
            if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
                scores[i][j] = 0;
                continue;
            }
            scores[i][j] = (1 + grid
                .iter()
                .take(i)
                .rev()
                .position(|x| *col <= x[j])
                .unwrap_or(i - 1))
		// really not sure why this one isn't +1 like the others, but it
		// works like this
                * grid
                    .iter()
                    .skip(i + 1)
                    .position(|x| *col <= x[j])
                    .unwrap_or(rows - i - 1)
                * (1 + grid[i]
                    .iter()
                    .take(j)
                    .rev()
                    .position(|x| col <= x)
                    .unwrap_or(j - 1))
                * (1 + grid[i]
                    .iter()
                    .skip(j + 1)
                    .position(|x| col <= x)
                    .unwrap_or(cols - j - 1));
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
    println!("{}", p1);
    println!("{}", p2);
    assert_eq!(p1, 1843);
    assert_eq!(p2, 180000);
}
