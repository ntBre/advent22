#![allow(unused)]
use std::{collections::HashSet, str::FromStr, string::ParseError};

use advent22::{load_input, load_sample};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rope {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::E),
            "L" => Ok(Direction::W),
            "D" => Ok(Direction::S),
            "U" => Ok(Direction::N),
            _ => panic!(),
        }
    }
}

impl Rope {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// single moves at a time
    fn mov(&mut self, dir: Direction) {
        use Direction::*;
        match dir {
            Direction::E => self.x += 1,
            Direction::N => self.y += 1,
            Direction::W => self.x -= 1,
            Direction::S => self.y -= 1,
            Direction::NE => {
                self.mov(N);
                self.mov(E);
            }
            Direction::NW => {
                self.mov(N);
                self.mov(W);
            }
            Direction::SE => {
                self.mov(S);
                self.mov(E);
            }
            Direction::SW => {
                self.mov(S);
                self.mov(W);
            }
        };
    }

    fn dist(&self, other: &Self) -> usize {
        let sx = self.x;
        let sy = self.y;
        let ox = other.x;
        let oy = other.y;
        f64::sqrt(((sx - ox).pow(2) + (sy - oy).pow(2)) as f64).round() as usize
    }

    /// return direction for `other` to move based on position of `self`
    fn compare(&self, other: &Self) -> Option<Direction> {
        use Direction::*;
        if self.dist(other) <= 1 {
            return None;
        }
        if self.x == other.x {
            if self.y > other.y {
                return Some(N);
            } else {
                return Some(S);
            }
        }

        if self.y == other.y {
            if self.x > other.x {
                return Some(E);
            } else {
                return Some(W);
            }
        }

        if self.x > other.x {
            if self.y > other.y {
                Some(NE)
            } else {
                Some(SE)
            }
        } else if self.y > other.y {
            Some(NW)
        } else {
            Some(SW)
        }
    }
}

fn inner(s: &str, l: usize) -> usize {
    let mut ropes = vec![Rope::new(0, 0); l];
    let mut counter = HashSet::new();
    counter.insert((ropes[l - 1].x, ropes[l - 1].y));
    for line in s.lines() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        let d = Direction::from_str(sp[0]).unwrap();
        let m = usize::from_str(sp[1]).unwrap();
        for _ in 0..m {
            // move the head
            ropes[0].mov(d);
            let mut head = ropes[0];
            for (i, tail) in ropes[1..].iter_mut().enumerate() {
                if let Some(dir) = head.compare(tail) {
                    tail.mov(dir);
                }
                head = *tail;
            }
            counter.insert((ropes[l - 1].x, ropes[l - 1].y));
        }
    }
    counter.len()
}

fn main() {
    let s = load_input();
    let p1 = inner(&s, 2);
    let p2 = inner(&s, 10);
    println!("{}", p1);
    println!("{}", p2);
    assert_eq!(p1, 6011);
    assert_eq!(p2, 2419);
}
