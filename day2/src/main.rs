use advent22::*;
use std::{cmp::Ordering, str::FromStr, string::ParseError};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Score {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Score {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> std::option::Option<std::cmp::Ordering> {
        Some(if self == other {
            Ordering::Equal
        } else if self.greater() == *other {
            Ordering::Less
        } else {
            Ordering::Greater
        })
    }
}

impl Score {
    fn score(&self) -> usize {
        match self {
            Score::Rock => 1,
            Score::Paper => 2,
            Score::Scissors => 3,
        }
    }

    /// return the variant greater than `self`
    fn greater(&self) -> Self {
        match self {
            Score::Rock => Self::Paper,
            Score::Paper => Self::Scissors,
            Score::Scissors => Self::Rock,
        }
    }

    /// return the variant less than `self`
    fn less(&self) -> Self {
        match self {
            Score::Rock => Self::Scissors,
            Score::Paper => Self::Rock,
            Score::Scissors => Self::Paper,
        }
    }
}

impl FromStr for Score {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => panic!("failed to match {s}"),
        }
    }
}

fn score(our: Score, their: Score) -> (usize, usize) {
    let (o, t) = if our == their {
        (3, 3)
    } else if our < their {
        (0, 6)
    } else {
        (6, 0)
    };
    (o + our.score(), t + their.score())
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => todo!(),
        }
    }
}

fn part1(s: &str) -> usize {
    let mut tot = 0;
    for line in s.lines() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        let theirs = Score::from_str(sp[0]).unwrap();
        let ours = Score::from_str(sp[1]).unwrap();
        let score = score(ours, theirs);
        tot += score.0;
    }
    tot
}

fn part2(s: &str) -> usize {
    let mut tot = 0;
    for line in s.lines() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        let theirs = Score::from_str(sp[0]).unwrap();
        let outcome = Outcome::from_str(sp[1]).unwrap();
        let ours = match (theirs, outcome) {
            (t, Outcome::Draw) => t,
            (t, Outcome::Win) => t.greater(),
            (t, Outcome::Lose) => t.less(),
        };
        let score = score(ours, theirs);
        tot += score.0;
    }
    tot
}

fn main() {
    let s = load_input();
    let p1 = part1(&s);
    let p2 = part2(&s);
    assert_eq!(p1, 11449);
    assert_eq!(p2, 13187);
    println!("{p1}");
    println!("{p2}");
}
