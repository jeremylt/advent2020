// Constants
const NUMBER_DASHES: usize = 80;

// -----------------------------------------------------------------------------
// Modules
// -----------------------------------------------------------------------------
mod day01;
mod day02;
mod file;
mod output;

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
    pub(crate) use crate::{file, output, Results, NUMBER_DASHES};
    pub(crate) use colored::*;
    pub(crate) use std::fs::File;
    pub(crate) use std::io::{BufRead, BufReader};
    pub(crate) use std::time::Instant;
}

// -----------------------------------------------------------------------------
// Main Driver
// -----------------------------------------------------------------------------
fn main() {
    // Each day
    output::print_header();
    let mut times: Vec<u128> = Vec::with_capacity(25);
    times.push(day01::run().time);
    times.push(day02::run().time);

    // Day comparison
    output::print_header();
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
    output::print_header();
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
