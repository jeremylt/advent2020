//! Day 15;

use crate::prelude::*;
use rustc_hash::FxHashMap;

// Constant
const YEAR: usize = 2020;
const CAPACITY: usize = 30000000;

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
    let mut said = [0; YEAR + 1];
    let number_starters = values.len();
    values
        .iter()
        .take(number_starters - 1)
        .enumerate()
        .for_each(|(i, &value)| {
            said[value] = i + 1;
        });

    let mut current = values[number_starters - 1];
    (number_starters..YEAR).for_each(|i| {
        let last_said = said[current];
        said[current] = i;
        if last_said == 0 {
            current = 0;
        } else {
            current = i - last_said;
        }
    });
    let number_1 = current;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let start_part_2 = Instant::now();
    let mut said = FxHashMap::<u32, u32>::with_capacity_and_hasher(1024, Default::default());
    values
        .iter()
        .take(number_starters - 1)
        .enumerate()
        .for_each(|(i, &value)| {
            said.insert(value as u32, (i + 1) as u32);
        });

    let mut current = values[number_starters - 1] as u32;
    (number_starters..CAPACITY).for_each(|i| {
        let last_said = said.insert(current, i as u32);
        if last_said == None {
            current = 0;
        } else {
            current = i as u32 - last_said.unwrap();
        }
    });
    let number_2 = current;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        number_1 as i64,
        number_2 as i64,
        Timing::new(
            time_setup,
            time_part_1,
            time_part_2,
            std::time::Duration::new(0, 0),
        ),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(1, "Rambunctious Recitation");
    output::print_part(1, "🧝 Number", &format!("{}", results.part_1));
    output::print_part(2, "🧝 Number", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
