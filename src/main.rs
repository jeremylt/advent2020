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
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
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
const REPETITIONS: u32 = 5;
pub(crate) mod prelude {
    pub(crate) use crate::REPETITIONS;
    pub(crate) use crate::{output, Results, Timing};
    pub(crate) use colored::*;
    pub(crate) use std::time::Instant;
}

// -----------------------------------------------------------------------------
// Main Driver
// -----------------------------------------------------------------------------
fn main() {
    // Setup
    const DAYS: usize = 18;
    let runs = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
    ];
    let reports = [
        day01::report,
        day02::report,
        day03::report,
        day04::report,
        day05::report,
        day06::report,
        day07::report,
        day08::report,
        day09::report,
        day10::report,
        day11::report,
        day12::report,
        day13::report,
        day14::report,
        day15::report,
        day16::report,
        day17::report,
        day18::report,
    ];

    // Each day
    output::print_header();
    let mut day_results: [Vec<Results>; DAYS] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    for _ in 0..REPETITIONS {
        for (i, day) in runs.iter().enumerate() {
            day_results[i].push(day());
        }
    }
    let average_times: Vec<Timing> = day_results
        .iter()
        .map(|day| {
            day.iter().fold(
                Timing::new(
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                ),
                |acc, result| {
                    Timing::new(
                        acc.setup + result.times.setup / REPETITIONS,
                        acc.part_1 + result.times.part_1 / REPETITIONS,
                        acc.part_2 + result.times.part_2 / REPETITIONS,
                        acc.combined + result.times.combined / REPETITIONS,
                    )
                },
            )
        })
        .collect();
    for i in 0..DAYS {
        let result = day_results[i].first().unwrap();
        let timing = &average_times[i];
        reports[i](&Results::new(
            result.part_1,
            result.part_2,
            Timing::new(timing.setup, timing.part_1, timing.part_2, timing.combined),
        ));
    }

    // Day comparison
    output::print_header();
    let time_averages = average_times
        .iter()
        .map(|day| {
            if day.combined.as_nanos() > 1 {
                day.combined
            } else {
                day.setup + day.part_1 + day.part_2
            }
        })
        .collect::<Vec<_>>();
    let time_std_devs: Vec<f64> = time_averages
        .iter()
        .zip(day_results.iter())
        .map(|(averages, day)| {
            (day.iter().fold(0.0, |acc, repetition| {
                let current = if repetition.times.combined.as_nanos() > 1 {
                    repetition.times.combined.as_nanos()
                } else {
                    repetition.times.setup.as_nanos()
                        + repetition.times.part_1.as_nanos()
                        + repetition.times.part_2.as_nanos()
                };
                acc + ((averages.as_nanos() as f64 - current as f64) / 1000.0).powf(2.0)
                    / ((REPETITIONS - 1) as f64)
            }))
            .powf(0.5)
        })
        .collect();
    output::print_days_timing(&time_averages, &time_std_devs);
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
            assert!(
                ($results.times.setup + $results.times.part_1 + $results.times.part_2).as_millis()
                    < MAX_TIME
            );
        };
    }

    #[test]
    fn test_01() {
        let results = day01::run();
        test_day!(results, 326_211, 131_347_190);
    }

    #[test]
    fn test_02() {
        let results = day02::run();
        test_day!(results, 538, 489);
    }

    #[test]
    fn test_03() {
        let results = day03::run();
        test_day!(results, 176, 5_872_458_240);
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
        test_day!(results, 6_249, 3_103);
    }

    #[test]
    fn test_07() {
        let results = day07::run();
        test_day!(results, 332, 10_875);
    }

    #[test]
    fn test_08() {
        let results = day08::run();
        test_day!(results, 1_594, 758);
    }

    #[test]
    fn test_09() {
        let results = day09::run();
        test_day!(results, 756_008_079, 93_727_241);
    }

    #[test]
    fn test_10() {
        let results = day10::run();
        test_day!(results, 2_244, 3_947_645_370_368);
    }

    #[test]
    fn test_11() {
        let results = day11::run();
        test_day!(results, 2_263, 2_002);
    }

    #[test]
    fn test_12() {
        let results = day12::run();
        test_day!(results, 879, 18_107);
    }

    #[test]
    fn test_13() {
        let results = day13::run();
        test_day!(results, 2_045, 402_251_700_208_309);
    }

    #[test]
    fn test_14() {
        let results = day14::run();
        test_day!(results, 15_172_047_086_292, 4_197_941_339_968);
    }

    #[test]
    fn test_15() {
        let results = day15::run();
        assert_eq!(results.part_1, 763);
        assert_eq!(results.part_2, 1_876_406);
    }

    #[test]
    fn test_16() {
        let results = day16::run();
        test_day!(results, 26_988, 426_362_917_709);
    }

    #[test]
    fn test_17() {
        let results = day17::run();
        test_day!(results, 315, 1_520);
    }

    #[test]
    fn test_18() {
        let results = day18::run();
        test_day!(results, 6_811_433_855_019, 129_770_152_447_927);
    }
}

// -----------------------------------------------------------------------------
