use std::fs;
use std::io::{self, Read};

use itertools::Itertools;

use crate::error::AocSolveErrkind;
use crate::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn part1(&self) -> crate::solution::AocResult {
        let mut input = String::with_capacity(10000);
        let mut file = io::BufReader::new(fs::File::open("./src/day3/input.txt")?);
        file.read_to_string(&mut input)?;

        Ok(format!(
            "{}",
            input
                .lines()
                .map(|line| line.split_at(line.len() >> 1))
                .filter_map(|(s1, s2)| intersections1(s1.as_bytes(), s2.as_bytes()))
                .sum::<usize>()
        ))
    }

    fn part2(&self) -> crate::solution::AocResult {
        let mut input = String::with_capacity(10000);
        let mut file = io::BufReader::new(fs::File::open("./src/day3/input.txt")?);
        file.read_to_string(&mut input)?;

        Ok(format!(
            "{}",
            input
                .lines()
                .chunks(3)
                .into_iter()
                .map(|mut chunk| (chunk.next(), chunk.next(), chunk.next()))
                .filter_map(|(s1, s2, s3)| intersections2(
                    s1.unwrap().as_bytes(),
                    s2.unwrap().as_bytes(),
                    s3.unwrap().as_bytes()
                ))
                .sum::<usize>()
        ))
    }
}

fn score(c: u8) -> usize {
    if c >= b'a' && c <= b'z' {
        (c - b'a' + 1) as usize
    } else {
        (c - b'A' + 27) as usize
    }
}

fn intersections1(s1: &[u8], s2: &[u8]) -> Option<usize> {
    let mut record1: u64 = 0;
    let mut record2: u64 = 0;

    for c in s1 {
        record1 |= 1 << score(*c);
    }

    for c in s2 {
        record2 |= 1 << score(*c);
    }

    let intersection = record1 & record2;

    for i in 0..64 {
        if intersection & 1 << i == 1 << i {
            return Some(i);
        }
    }

    None
}

fn intersections2(s1: &[u8], s2: &[u8], s3: &[u8]) -> Option<usize> {
    let mut record1: u64 = 0;
    let mut record2: u64 = 0;
    let mut record3: u64 = 0;

    for c in s1 {
        record1 |= 1 << score(*c);
    }

    for c in s2 {
        record2 |= 1 << score(*c);
    }

    for c in s3 {
        record3 |= 1 << score(*c);
    }

    let intersection = record1 & record2 & record3;

    for i in 0..64 {
        if intersection & 1 << i == 1 << i {
            return Some(i);
        }
    }

    None
}
