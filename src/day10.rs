//! Day 10:
//! The only tricks in this one are using a mask array to sort the input and counting the
//! unique combinations from the end of the array.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Run
// -----------------------------------------------------------------------------
pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let buffer: String = std::fs::read_to_string("data/day10.txt").unwrap();

    // Read to vector
    let mut max = 0;
    let values: Vec<usize> = buffer
        .lines()
        .map(|line| {
            let value = line.parse().expect("failed to parse line");
            max = std::cmp::max(max, value);
            value
        })
        .collect();

    // Collect into mask array
    let mut mask: Vec<usize> = vec![0; max + 1];
    values.iter().for_each(|&value| mask[value] = 1);
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Look for pair
    let start_part_1 = Instant::now();
    let mut count_1 = [0, 0, 1];
    let mut last = 0;
    mask.iter().skip(1).for_each(|&value| {
        if value > 0 {
            count_1[last] += 1;
            last = 0;
        } else {
            last += 1;
        }
    });
    let product_1 = count_1[0] * count_1[2];
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let start_part_2 = Instant::now();
    mask.append(&mut vec![0, 0, 1]);
    (0..max).rev().for_each(|i| {
        if mask[i] > 0 {
            mask[i] = mask[i + 1] + mask[i + 2] + mask[i + 3]
        }
    });
    let count_2 = mask[1] + mask[2] + mask[3];
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        product_1 as i64,
        count_2 as i64,
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
    output::print_day(10, "Adapter Array");
    output::print_part(1, "🔌 Product", &format!("{}", results.part_1));
    output::print_part(2, "🔌 Combinations", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
