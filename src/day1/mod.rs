use std::fs;
use std::io::{self, Read};

use itertools::Itertools;

use crate::error::AocSolveErrkind;
use crate::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1(&self) -> crate::solution::AocResult {
        let mut input = String::with_capacity(10000);
        let mut file = io::BufReader::new(fs::File::open("./src/day1/input.txt")?);
        file.read_to_string(&mut input)?;

        Ok(format!(
            "{}",
            input
                .split("\n\n")
                .map(|chunks| {
                    chunks
                        .lines()
                        .filter_map(|line| line.parse::<u64>().ok())
                        .sum::<u64>()
                })
                .max()
                .ok_or(AocSolveErrkind::ErrWithNoInfo)?
        ))
    }

    fn part2(&self) -> crate::solution::AocResult {
        let mut input = String::with_capacity(10000);
        let mut file = io::BufReader::new(fs::File::open("./src/day1/input.txt")?);
        file.read_to_string(&mut input)?;

        Ok(format!(
            "{}",
            input
                .split("\n\n")
                .map(|chunks| {
                    chunks
                        .lines()
                        .filter_map(|line| line.parse::<u64>().ok())
                        .sum::<u64>()
                })
                .map(std::cmp::Reverse)
                .k_smallest(3)
                .map(|n| n.0)
                .sum::<u64>()
        ))
    }
}
