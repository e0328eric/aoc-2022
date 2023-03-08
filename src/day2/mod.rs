use std::fs;
use std::io::{self, Read};
use std::str::FromStr;

use crate::error::AocSolveErrkind;
use crate::Solution;

pub struct Day2;

#[derive(Debug)]
struct Part1Info {
    opponent: Hand,
    me: Hand,
}

#[derive(Debug)]
struct Part2Info {
    opponent: Hand,
    me: Outcome,
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Sissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Solution for Day2 {
    fn part1(&self) -> crate::solution::AocResult {
        let mut input = String::with_capacity(10000);
        let mut file = io::BufReader::new(fs::File::open("./src/day2/input.txt")?);
        file.read_to_string(&mut input)?;

        Ok(format!(
            "{}",
            input
                .lines()
                .map(|line| line.parse::<Part1Info>().unwrap().score())
                .sum::<usize>()
        ))
    }

    fn part2(&self) -> crate::solution::AocResult {
        let mut input = String::with_capacity(10000);
        let mut file = io::BufReader::new(fs::File::open("./src/day2/input.txt")?);
        file.read_to_string(&mut input)?;

        Ok(format!(
            "{}",
            input
                .lines()
                .map(|line| line.parse::<Part2Info>().unwrap().score())
                .sum::<usize>()
        ))
    }
}

impl Hand {
    fn is_win(&self, opponent: &Self) -> bool {
        matches!(
            (self, opponent),
            (Self::Rock, Self::Sissors) | (Self::Sissors, Self::Paper) | (Self::Paper, Self::Rock)
        )
    }

    fn outcome(&self, opponent: &Self) -> Outcome {
        if self == opponent {
            Outcome::Draw
        } else if self.is_win(opponent) {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Sissors => 3,
        }
    }
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn get_my_hand(&self, opponent: &Hand) -> Hand {
        match (self, opponent) {
            (Self::Win, Hand::Rock) => Hand::Paper,
            (Self::Win, Hand::Paper) => Hand::Sissors,
            (Self::Win, Hand::Sissors) => Hand::Rock,
            (Self::Draw, Hand::Rock) => Hand::Rock,
            (Self::Draw, Hand::Paper) => Hand::Paper,
            (Self::Draw, Hand::Sissors) => Hand::Sissors,
            (Self::Lose, Hand::Rock) => Hand::Sissors,
            (Self::Lose, Hand::Paper) => Hand::Rock,
            (Self::Lose, Hand::Sissors) => Hand::Paper,
        }
    }
}

impl Part1Info {
    fn score(&self) -> usize {
        self.me.outcome(&self.opponent).score() + self.me.score()
    }
}

impl FromStr for Part1Info {
    type Err = AocSolveErrkind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(char::is_whitespace);

        let (Some(opponent), Some(me), None) = (iter.next(), iter.next(), iter.next()) else {
            return Err(AocSolveErrkind::ParseFailed);
        };

        let opponent = match opponent {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Sissors,
            _ => return Err(AocSolveErrkind::ParseFailed),
        };

        let me = match me {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Sissors,
            _ => return Err(AocSolveErrkind::ParseFailed),
        };

        Ok(Self { opponent, me })
    }
}

impl Part2Info {
    fn score(&self) -> usize {
        self.me.get_my_hand(&self.opponent).score() + self.me.score()
    }
}

impl FromStr for Part2Info {
    type Err = AocSolveErrkind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(char::is_whitespace);

        let (Some(opponent), Some(me), None) = (iter.next(), iter.next(), iter.next()) else {
            return Err(AocSolveErrkind::ParseFailed);
        };

        let opponent = match opponent {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Sissors,
            _ => return Err(AocSolveErrkind::ParseFailed),
        };

        let me = match me {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => return Err(AocSolveErrkind::ParseFailed),
        };

        Ok(Self { opponent, me })
    }
}
