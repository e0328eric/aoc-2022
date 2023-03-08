use crate::error;

pub type AocResult = error::Result<String>;

pub trait Solution {
    fn part1(&self) -> AocResult;
    fn part2(&self) -> AocResult;
}
