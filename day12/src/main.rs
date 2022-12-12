use advent22::*;
use std::{collections::HashMap, ops::Index};

fn to_height(c: char) -> usize {
    let map: HashMap<char, usize> = ('a'..='z').zip(0..26).collect();
    map[&c]
}

struct Grid(Vec<Vec<usize>>);
impl Grid {
    fn shape(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}
impl Index<(usize, usize)> for Grid {
    type Output = usize;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.0[x][y]
    }
}
type Point = (usize, usize);

fn load_grid(s: String) -> (Grid, Point, Point) {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in s.lines().enumerate() {
        let v: Vec<_> = line
            .chars()
            .enumerate()
            .map(|(j, c)| {
                if c == 'S' {
                    start = (i, j);
                    0
                } else if c == 'E' {
                    end = (i, j);
                    25
                } else {
                    to_height(c)
                }
            })
            .collect();
        grid.push(v);
    }
    (Grid(grid), start, end)
}

fn generate_moves(pos: Point, grid: &Grid) -> Vec<Point> {
    let (x, y) = pos;
    let h = grid[pos];
    let mut ret = Vec::new();
    if x > 0 {
        let h2 = grid[(x - 1, y)];
        if h2 <= h + 1 {
            ret.push((x - 1, y));
        }
    }
    if y > 0 {
        let h2 = grid[(x, y - 1)];
        if h2 <= h + 1 {
            ret.push((x, y - 1));
        }
    }

    let (mx, my) = grid.shape();
    if x < mx - 1 {
        let h2 = grid[(x + 1, y)];
        if h2 <= h + 1 {
            ret.push((x + 1, y));
        }
    }
    if y < my - 1 {
        let h2 = grid[(x, y + 1)];
        if h2 <= h + 1 {
            ret.push((x, y + 1));
        }
    }
    ret
}

fn main() {
    let s = load_sample();
    let (grid, start, end) = load_grid(s);
    // build tree of legal moves at every position
    let mut moves = generate_moves(start, &grid);
    let mut round = 1;
    'outer: loop {
        let mut new_moves = Vec::new();
        for mov in std::mem::take(&mut moves) {
            if mov == end {
                break 'outer;
            }
            new_moves.extend(generate_moves(mov, &grid));
        }
        moves = new_moves;
        round += 1;
        dbg!(round, moves.len());
    }
    dbg!(round);
    // find paths through tree terminating at end
    // choose the shortest one
}
