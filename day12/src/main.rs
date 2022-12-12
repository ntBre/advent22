use advent22::*;
use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    fn new(data: T) -> Self {
        Self {
            nodes: vec![Node {
                parent: None,
                children: Vec::new(),
                data,
            }],
        }
    }

    /// push a [Node] containing `data` into the tree and return its index in the tree
    fn push(&mut self, idx: usize, data: T) -> usize {
        let l = self.nodes.len();
        self.nodes.push(Node {
            parent: Some(idx),
            children: Vec::new(),
            data,
        });
        self.nodes[idx].children.push(l);
        l
    }
}

#[derive(Debug)]
struct Node<T> {
    /// index into containing tree. None for root of tree
    parent: Option<usize>,
    /// also indices into the containing tree
    children: Vec<usize>,
    data: T,
}

impl<T> Node<T> {
    fn inner(&self) -> &T {
        &self.data
    }
}

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

fn generate_moves(
    tree: &Tree<Point>,
    node: &Node<Point>,
    grid: &Grid,
) -> Vec<Point> {
    let pos = node.inner();
    let (x, y) = pos;
    let h = grid[*pos];
    let mut ret = Vec::new();
    if *x > 0 {
        let h2 = grid[(*x - 1, *y)];
        if h2 <= h + 1 {
            ret.push((*x - 1, *y));
        }
    }
    if *y > 0 {
        let h2 = grid[(*x, *y - 1)];
        if h2 <= h + 1 {
            ret.push((*x, *y - 1));
        }
    }

    let (mx, my) = grid.shape();
    if *x < mx - 1 {
        let h2 = grid[(*x + 1, *y)];
        if h2 <= h + 1 {
            ret.push((*x + 1, *y));
        }
    }
    if *y < my - 1 {
        let h2 = grid[(*x, *y + 1)];
        if h2 <= h + 1 {
            ret.push((*x, *y + 1));
        }
    }
    ret.into_iter()
        .filter(|p| {
            let mut cur = node.parent;
            while let Some(parent) = cur {
                if p == tree.nodes[parent].inner() {
                    return false;
                }
                cur = tree.nodes[parent].parent;
            }
            true
        })
        .collect()
}

fn main() {
    let s = load_sample();
    let (grid, start, end) = load_grid(s);
    let mut tree = Tree::new(start);
    // build tree of legal moves at every position
    let mut to_visit = vec![0];
    let mut round = 1;
    'outer: loop {
        let now = std::time::Instant::now();
        let tv = std::mem::take(&mut to_visit);
        for cur in tv {
            let moves = generate_moves(&tree, &tree.nodes[cur], &grid);
            for mov in moves {
                if mov == end {
                    break 'outer;
                }
                to_visit.push(tree.push(cur, mov));
            }
        }
        println!(
            "finished round {round} after {:.1}",
            now.elapsed().as_millis() as f64 / 1000.0
        );
        round += 1;
    }
    dbg!(round);
    // dbg!(round);
    // find paths through tree terminating at end
    // choose the shortest one
}
