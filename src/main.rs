// -----------------------------------------------------------------------------
// Modules
// -----------------------------------------------------------------------------
mod day01;
mod day02;
mod day03;
mod day04;
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
pub(crate) mod prelude {
    pub(crate) use crate::{file, output, Results};
    pub(crate) use colored::*;
    pub(crate) use std::fs::File;
    pub(crate) use std::io::{BufRead, BufReader};
    pub(crate) use std::time::Instant;
}

// -----------------------------------------------------------------------------
// Main Driver
// -----------------------------------------------------------------------------
fn main() {
    // Setup
    const PRINT_OUTPUT: bool = true;
    let mut times: Vec<u128> = Vec::with_capacity(25);
    let days = [day01::run, day02::run, day03::run, day04::run];

    // Each day
    output::print_header();
    for day in &days {
        times.push(day(PRINT_OUTPUT).time);
    }

    // Day comparison
    output::print_header();
    output::print_days_timing(&times);
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

    #[test]
    fn test_04() {
        test_day!(day04::run(NO_OUTPUT), 182, 109);
    }
}

// -----------------------------------------------------------------------------
