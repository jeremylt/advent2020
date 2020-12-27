//! Day 15:
//! Nothing clever here. I am using a vec for the dense portion of the integers and a
//! hash map for the sparse portion of the integers.

use crate::prelude::*;
use rustc_hash::FxHashMap;

// Constants
const YEAR: usize = 2020;
const REALLY_BIG: usize = 30_000_000;
const BREAKPOINT: usize = 3_000_000;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
macro_rules! turns_since_said {
    ($said:expr, $turn:expr, $current:expr) => {
        ($turn as u32).saturating_sub(std::mem::replace(
            &mut $said[$current as usize],
            $turn as u32,
        ))
    };
}

#[inline]
fn part_1(n: usize, starters: &Vec<usize>) -> u32 {
    // Setup
    let mut said = vec![u32::MAX; n + 1];
    let number_starters = starters.len();
    starters
        .iter()
        .take(number_starters - 1)
        .enumerate()
        .for_each(|(i, &value)| {
            said[value] = (i + 1) as u32;
        });

    // Iterate to nth
    (number_starters..n).fold(starters[number_starters - 1] as u32, |current, i| {
        turns_since_said!(said, i, current)
    })
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
macro_rules! turns_since_said_big {
    ($said_big:expr, $turn:expr, $current:expr) => {
        if let Some(previous) = $said_big.insert($current, $turn as u32) {
            ($turn as u32).saturating_sub(previous)
        } else {
            0
        }
    };
}

#[inline]
fn part_2(n: usize, starters: &Vec<usize>) -> u32 {
    // Setup
    let mut said = vec![u32::MAX; BREAKPOINT].into_boxed_slice();
    let breakpoint = said.len();
    let mut said_big =
        FxHashMap::<u32, u32>::with_capacity_and_hasher(BREAKPOINT / 16 * 9, Default::default());
    let number_starters = starters.len();
    starters
        .iter()
        .take(number_starters - 1)
        .enumerate()
        .for_each(|(i, &value)| {
            said[value] = (i + 1) as u32;
        });

    // Lower portion of range
    let lower = (number_starters..breakpoint)
        .fold(starters[number_starters - 1] as u32, |current, i| {
            turns_since_said!(said, i, current)
        });

    // Upper portion of range
    (breakpoint..n).fold(lower, |current, i| {
        if (current as usize) < breakpoint {
            turns_since_said!(said, i, current)
        } else {
            turns_since_said_big!(said_big, i, current)
        }
    })
}

// -----------------------------------------------------------------------------
// Combined
// -----------------------------------------------------------------------------
#[inline]
fn combined(first: usize, second: usize, starters: &Vec<usize>) -> (u32, u32) {
    // Setup
    let mut said = vec![u32::MAX; BREAKPOINT].into_boxed_slice();
    let breakpoint = said.len();
    assert!(first < breakpoint);
    let mut said_big =
        FxHashMap::<u32, u32>::with_capacity_and_hasher(BREAKPOINT / 16 * 9, Default::default());
    let number_starters = starters.len();
    starters
        .iter()
        .take(number_starters - 1)
        .enumerate()
        .for_each(|(i, &value)| {
            said[value] = (i + 1) as u32;
        });
    // First range
    let result_1 = (number_starters..first)
        .fold(starters[number_starters - 1] as u32, |current, i| {
            turns_since_said!(said, i, current)
        });

    // Lower portion of second range
    let lower =
        (first..breakpoint).fold(result_1, |current, i| turns_since_said!(said, i, current));

    // Upper portion of second range
    let result_2 = (breakpoint..second).fold(lower, |current, i| {
        if (current as usize) < breakpoint {
            turns_since_said!(said, i, current)
        } else {
            turns_since_said_big!(said_big, i, current)
        }
    });

    (result_1, result_2)
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
    let buffer: String = std::fs::read_to_string("data/day15.txt").unwrap();

    // Read to vector
    let values: Vec<usize> = buffer
        .trim()
        .split(',')
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find 2020th number
    let start_part_1 = Instant::now();
    let number_1 = part_1(YEAR, &values);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find 30,000,000th number
    let start_part_2 = Instant::now();
    let number_2 = part_2(REALLY_BIG, &values);
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let values: Vec<usize> = buffer
        .trim()
        .split(',')
        .map(|line| line.parse().expect("failed to parse line"))
        .collect();
    let (combined_1, combined_2) = combined(YEAR, REALLY_BIG, &values);
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, number_1);
    assert_eq!(combined_2, number_2);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        number_1 as i64,
        number_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(15, "Rambunctious Recitation");
    output::print_part(1, "🧝 Number", &format!("{}", results.part_1));
    output::print_part(2, "🧝 Number", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
