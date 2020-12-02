#[macro_use]
extern crate lazy_static;

// Constants
const NUMBER_DASHES: usize = 80;

// -----------------------------------------------------------------------------
// Modules
// -----------------------------------------------------------------------------
mod day01;
mod day02;

// -----------------------------------------------------------------------------
// Results struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Results {
    part1: i32,
    part2: i32,
    time: u128,
}

// -----------------------------------------------------------------------------
// Prelude
// -----------------------------------------------------------------------------
use crate::prelude::*;
pub(crate) mod prelude {
    pub(crate) use crate::Results;
    pub(crate) use crate::NUMBER_DASHES;
    pub(crate) use colored::*;
    pub(crate) use std::fs::File;
    pub(crate) use std::io::{BufRead, BufReader};
    pub(crate) use std::time::Instant;
}

// -----------------------------------------------------------------------------
// Print header
// -----------------------------------------------------------------------------
fn header() {
    println!("{}", "-".repeat(NUMBER_DASHES).green().bold());
    println!(
        "{} {} {}",
        "-".repeat(NUMBER_DASHES / 2 - 10).red().bold(),
        "Advent of Code 2020".bold(),
        "-".repeat(NUMBER_DASHES / 2 - 11).red().bold()
    );
    println!("{}", "-".repeat(NUMBER_DASHES).green().bold());
}

// -----------------------------------------------------------------------------
// Main Driver
// -----------------------------------------------------------------------------
fn main() {
    // Each day
    header();
    let mut times: Vec<u128> = Vec::with_capacity(25);
    times.push(day01::run().time);
    times.push(day02::run().time);

    // Day comparison
    header();
    println!("- {}", "Timing Comparison".bold());
    let longest: f64 = (*times.iter().max().unwrap()) as f64;
    for (i, &time) in times.iter().enumerate() {
        let length = std::cmp::max(
            1,
            ((NUMBER_DASHES - 7) as f64 * (time as f64 / longest)) as usize,
        );
        let dashes = "-".repeat(length);
        println!(
            "Dec {:02} {}",
            i + 1,
            if i % 2 == 0 {
                dashes.red()
            } else {
                dashes.green()
            }
        );
    }

    header();
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    const MAX_TIME: u128 = 250000000;

    #[test]
    fn test01() {
        let results = day01::run();
        assert_eq!(results.part1, 326211);
        assert_eq!(results.part2, 131347190);
        assert!(results.time < MAX_TIME);
    }

    #[test]
    fn test02() {
        let results = day02::run();
        assert_eq!(results.part1, 538);
        assert_eq!(results.part2, 489);
        assert!(results.time < MAX_TIME);
    }
}

// -----------------------------------------------------------------------------
