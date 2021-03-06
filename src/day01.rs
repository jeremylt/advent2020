//! Day 1:
//! This solution uses a binary mask array that contains the values between 1 and 2020
//! found in the challenge input. I read the values into a mask array and load the values
//! into a vector at the same time to reduce the memory movement.

use crate::prelude::*;

// Constant
const YEAR: usize = 2020;

// -----------------------------------------------------------------------------
// Find pair of flagged indices that sum to length of mask array
// -----------------------------------------------------------------------------
#[inline(always)]
fn find_two(array: &[bool]) -> Option<i32> {
    array
        .iter()
        .enumerate()
        .zip(array.iter().rev())
        .find_map(|((i, a), b)| if a & b { Some(i as i32) } else { None })
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(mask: &[bool]) -> (i32, i32) {
    match find_two(&mask) {
        Some(index) => (index, YEAR as i32 - index),
        None => panic!("No pair found"),
    }
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(values: &[usize], mask: &[bool]) -> (i32, i32, i32) {
    for value in values {
        let remainder = YEAR - *value;
        let index = find_two(&mask[0..=remainder]);
        if index != None {
            let triple = (
                *value as i32,
                index.unwrap(),
                YEAR as i32 - *value as i32 - index.unwrap(),
            );
            return triple;
        }
    }
    panic!("No triple found");
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
    let buffer: String = std::fs::read_to_string("data/day01.txt").unwrap();

    // Read to vector
    let mut mask = [false; YEAR + 1];
    let values: Vec<usize> = buffer
        .lines()
        .map(|line| {
            // Read to array and mask at same time
            let value = line.parse().expect("failed to parse line");
            mask[value] = true;
            value
        })
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Look for pair
    let start_part_1 = Instant::now();
    let tuple = part_1(&mask);
    let product_1 = tuple.0 * tuple.1;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let start_part_2 = Instant::now();
    let triple = part_2(&values, &mask);
    let product_2 = triple.0 * triple.1 * triple.2;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        product_1 as i64,
        product_2 as i64,
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
    output::print_day(1, "Report Repair");
    output::print_part(1, "📄 Product", &format!("{}", results.part_1));
    output::print_part(2, "📄 Product", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
