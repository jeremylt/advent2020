#[macro_use]
extern crate lazy_static;

// -----------------------------------------------------------------------------
// Modules
// -----------------------------------------------------------------------------
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod output;

// -----------------------------------------------------------------------------
// Results struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Results {
    part_1: i64,
    part_2: i64,
    times: Timing,
}

impl Results {
    fn new(part_1: i64, part_2: i64, times: Timing) -> Self {
        Self {
            part_1,
            part_2,
            times,
        }
    }
}

// -----------------------------------------------------------------------------
// Timing struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Timing {
    setup: std::time::Duration,
    part_1: std::time::Duration,
    part_2: std::time::Duration,
    combined: std::time::Duration,
}

impl Timing {
    fn new(
        setup: std::time::Duration,
        part_1: std::time::Duration,
        part_2: std::time::Duration,
        combined: std::time::Duration,
    ) -> Self {
        Self {
            setup,
            part_1,
            part_2,
            combined,
        }
    }
}

// -----------------------------------------------------------------------------
// Prelude
// -----------------------------------------------------------------------------
pub(crate) mod prelude {
    pub(crate) use crate::{output, Results, Timing};
    pub(crate) use colored::*;
    pub(crate) use std::time::Instant;
}

// -----------------------------------------------------------------------------
// Main Driver
// -----------------------------------------------------------------------------
fn main() {
    // Setup
    const REPETITIONS: u32 = 20;
    let mut summary: Vec<std::time::Duration> = Vec::with_capacity(25);
    let runs = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
    ];
    let reports = [
        day01::report,
        day02::report,
        day03::report,
        day04::report,
        day05::report,
        day06::report,
        day07::report,
    ];

    // Each day
    output::print_header();
    for (day, report) in runs.iter().zip(&reports) {
        let result = day();
        let mut times = Timing::new(
            result.times.setup / REPETITIONS,
            result.times.part_1 / REPETITIONS,
            result.times.part_2 / REPETITIONS,
            result.times.combined / REPETITIONS,
        );
        for _ in 0..REPETITIONS - 1 {
            let result = day();
            times.setup += result.times.setup / REPETITIONS;
            times.part_1 += result.times.part_1 / REPETITIONS;
            times.part_2 += result.times.part_2 / REPETITIONS;
            times.combined += result.times.combined / REPETITIONS;
        }
        report(&Results::new(
            result.part_1,
            result.part_2,
            Timing::new(times.setup, times.part_1, times.part_2, times.combined),
        ));
        summary.push(times.combined);
    }

    // Day comparison
    output::print_header();
    output::print_days_timing(&summary);
    output::print_header();
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    const MAX_TIME: u128 = 250;
    macro_rules! test_day {
        ($results:expr, $part_1:expr, $part_2:expr) => {
            assert_eq!($results.part_1, $part_1);
            assert_eq!($results.part_2, $part_2);
            assert!($results.times.combined.as_millis() < MAX_TIME);
        };
    }

    #[test]
    fn test_01() {
        let results = day01::run();
        test_day!(results, 326211, 131347190);
    }

    #[test]
    fn test_02() {
        let results = day02::run();
        test_day!(results, 538, 489);
    }

    #[test]
    fn test_03() {
        let results = day03::run();
        test_day!(results, 176, 5872458240);
    }

    #[test]
    fn test_04() {
        let results = day04::run();
        test_day!(results, 182, 109);
    }

    #[test]
    fn test_05() {
        let results = day05::run();
        test_day!(results, 892, 625);
    }

    #[test]
    fn test_06() {
        let results = day06::run();
        test_day!(results, 6249, 3103);
    }

    #[test]
    fn test_07() {
        let results = day07::run();
        test_day!(results, 332, 10875);
    }
}

// -----------------------------------------------------------------------------
