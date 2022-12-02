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
        Some(match (self, other) {
            (Score::Rock, Score::Rock) => Ordering::Equal,
            (Score::Rock, Score::Paper) => Ordering::Less,
            (Score::Rock, Score::Scissors) => Ordering::Greater,
            (Score::Paper, Score::Rock) => Ordering::Greater,
            (Score::Paper, Score::Paper) => Ordering::Equal,
            (Score::Paper, Score::Scissors) => Ordering::Less,
            (Score::Scissors, Score::Rock) => Ordering::Less,
            (Score::Scissors, Score::Paper) => Ordering::Greater,
            (Score::Scissors, Score::Scissors) => Ordering::Equal,
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
}

impl FromStr for Score {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
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

fn main() {
    let s = load_input();
    let mut tot = 0;
    for line in s.lines() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        let theirs = Score::from_str(sp[0]).unwrap();
        let outcome = Outcome::from_str(sp[1]).unwrap();
        use Score::*;
        let ours = match (theirs, outcome) {
            (Score::Rock, Outcome::Win) => Paper,
            (Score::Rock, Outcome::Lose) => Scissors,
            (Score::Rock, Outcome::Draw) => Rock,
            (Score::Paper, Outcome::Win) => Scissors,
            (Score::Paper, Outcome::Lose) => Rock,
            (Score::Paper, Outcome::Draw) => Paper,
            (Score::Scissors, Outcome::Win) => Rock,
            (Score::Scissors, Outcome::Lose) => Paper,
            (Score::Scissors, Outcome::Draw) => Scissors,
        };
        let score = score(ours, theirs);
        tot += score.0;
    }
    println!("{tot}");
}
