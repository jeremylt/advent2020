// Constants
const NUMBER_DASHES: usize = 80;

// -----------------------------------------------------------------------------
// Modules
// -----------------------------------------------------------------------------
mod day01;
mod day02;
mod day03;
mod file;
mod output;

// -----------------------------------------------------------------------------
// Results struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Results {
    part_1: i64,
    part_2: i64,
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
    const PRINT_OUTPUT: bool = true;
    let days = [day01::run, day02::run, day03::run];
    for day in &days {
        times.push(day(PRINT_OUTPUT).time);
    }

    // Day comparison
    output::print_header();
    println!("- {}", "Timing Comparison".bold());
    let longest: f64 = (*times.iter().max().unwrap()) as f64;
    for (i, &time) in times.iter().enumerate() {
        let part_length = std::cmp::max(
            1,
            ((NUMBER_DASHES - 7) as f64 * (time as f64 / longest)) as usize,
        );
        let dashes = "-".repeat(part_length);
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
    const NO_OUTPUT: bool = false;
    const MAX_TIME: u128 = 250000000;
    macro_rules! test_day {
        ($results:expr, $part_1:expr, $part_2:expr) => {
            assert_eq!($results.part_1, $part_1);
            assert_eq!($results.part_2, $part_2);
            assert!($results.time < MAX_TIME);
        };
    }

    #[test]
    fn test_01() {
        test_day!(day01::run(NO_OUTPUT), 326211, 131347190);
    }

    #[test]
    fn test_02() {
        test_day!(day02::run(NO_OUTPUT), 538, 489);
    }

    #[test]
    fn test_03() {
        test_day!(day03::run(NO_OUTPUT), 176, 5872458240);
    }
}

// -----------------------------------------------------------------------------
