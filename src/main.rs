mod error;
mod solution;

mod day1;
mod day2;
mod day3;
mod day4;

pub use solution::Solution;

fn main() -> error::Result<()> {
    let days: Vec<Box<dyn Solution>> = vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
        Box::new(day4::Day4),
    ];

    for (idx, day) in days.into_iter().enumerate() {
        println!("=====  Day {day_num}  =====", day_num = idx + 1);
        println!("part1: {}", day.part1()?);
        println!("part2: {}", day.part2()?);
        println!("");
    }

    Ok(())
}
