#[macro_use]
extern crate lazy_static;
const NUMBER_DASHES: usize = 41;
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
// Main Driver
// -----------------------------------------------------------------------------
fn main() {
    println!(
        "{}",
        "-----------------------------------------".green().bold()
    );
    println!(
        "{} {} {}",
        "----------".red().bold(),
        "Advent of Code 2020".bold(),
        "----------".red().bold()
    );
    println!(
        "{}",
        "-----------------------------------------".green().bold()
    );
    day01::run();
    day02::run();
    println!(
        "{} {} {}",
        "----------".red().bold(),
        "Advent of Code 2020".bold(),
        "----------".red().bold()
    );
    println!(
        "{}",
        "-----------------------------------------".green().bold()
    );
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let results = day01::run();
        assert_eq!(results.part1, 326211);
        assert_eq!(results.part2, 131347190);
        assert!(results.time < 250);
    }

    #[test]
    fn test02() {
        let results = day02::run();
        assert_eq!(results.part1, 538);
        assert_eq!(results.part2, 489);
        assert!(results.time < 250);
    }
}

// -----------------------------------------------------------------------------
