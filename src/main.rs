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
        report(&result);

        let mut times = Timing {
            setup: result.times.setup / REPETITIONS,
            part_1: result.times.part_1 / REPETITIONS,
            part_2: result.times.part_2 / REPETITIONS,
            combined: result.times.combined / REPETITIONS,
        };
        for _ in 0..REPETITIONS {
            let result = day();
            times = Timing {
                setup: times.setup + result.times.setup / REPETITIONS,
                part_1: times.part_1 + result.times.part_1 / REPETITIONS,
                part_2: times.part_2 + result.times.part_2 / REPETITIONS,
                combined: times.combined + result.times.combined / REPETITIONS,
            }
        }
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
        test_day!(day01::run(), 326211, 131347190);
    }

    #[test]
    fn test_02() {
        test_day!(day02::run(), 538, 489);
    }

    #[test]
    fn test_03() {
        test_day!(day03::run(), 176, 5872458240);
    }

    #[test]
    fn test_04() {
        test_day!(day04::run(), 182, 109);
    }

    #[test]
    fn test_05() {
        test_day!(day05::run(), 892, 625);
    }

    #[test]
    fn test_06() {
        test_day!(day06::run(), 6249, 3103);
    }

    #[test]
    fn test_07() {
        test_day!(day07::run(), 332, 10875);
    }
}

// -----------------------------------------------------------------------------
