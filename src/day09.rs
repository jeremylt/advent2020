//! Day 9:
//! The parts are difficult (or impossible) to combine in this problem. Some approaches,
//! such as using hash sets, offer better theoretical accuracy, but in practice I am
//! seeing that using fixed sized arrays is offering the compiler plenty of chances to
//! produce fast code.

use crate::prelude::*;

// Constant
const WINDOW: usize = 25;

// -----------------------------------------------------------------------------
// Check for pair that sum to target in current range
// -----------------------------------------------------------------------------
fn find_two(target: &i64, values: &[i64; WINDOW]) -> Option<i64> {
    values.iter().enumerate().find_map(|(i, value)| {
        let search = target - value;
        if values.iter().skip(i + 1).any(|second| *second == search) {
            Some(*value)
        } else {
            None
        }
    })
}

// -----------------------------------------------------------------------------
// Run
// -----------------------------------------------------------------------------
pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let buffer: String = std::fs::read_to_string("data/day09.txt").unwrap();

    // Read to vector
    let values: Vec<i64> = buffer
        .lines()
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let mut current_values: [i64; WINDOW] = [0; WINDOW];
    values
        .iter()
        .take(WINDOW)
        .enumerate()
        .for_each(|(i, &value)| current_values[i] = value);
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Look for pair
    let start_part_1 = Instant::now();
    let value_1 = values
        .iter()
        .skip(WINDOW)
        .enumerate()
        .find_map(|(i, value)| match find_two(value, &current_values) {
            Some(_) => {
                current_values[i % WINDOW] = *value;
                None
            }
            None => Some(*value),
        })
        .unwrap();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for range
    let start_part_2 = Instant::now();
    let (lower, upper) = values
        .iter()
        .skip(WINDOW)
        .enumerate()
        .find_map(|(i, value)| {
            let mut sum = *value;
            let j = values
                .iter()
                .skip(i + WINDOW + 1)
                .enumerate()
                .find_map(|(j, next)| {
                    sum += next;
                    if sum >= value_1 {
                        Some(j)
                    } else {
                        None
                    }
                })
                .unwrap();
            if sum == value_1 {
                Some((i, j))
            } else {
                None
            }
        })
        .unwrap();
    let (min, max) = values
        .iter()
        .skip(WINDOW + lower)
        .take(upper)
        .fold((value_1, 0), |acc, value| {
            (std::cmp::min(acc.0, *value), std::cmp::max(acc.1, *value))
        });
    let sum_2 = min + max;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        value_1 as i64,
        sum_2 as i64,
        Timing::new(
            time_setup,
            time_part_1,
            time_part_2,
            time_setup + time_part_1 + time_part_2,
        ),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(9, "Encoding Error");
    output::print_part(1, "💾 Invalid", &format!("{}", results.part_1));
    output::print_part(2, "💾 Sum", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
