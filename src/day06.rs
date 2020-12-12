//! Day 6:
//! This is a good example of how using the bytes array for each string can speed up
//! comparisons and parsing. Of note here, it is faster to map and then fold in the
//! combined section.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Parse line to array
// -----------------------------------------------------------------------------
fn to_array(responses: &str) -> Result<[usize; 27], std::num::ParseIntError> {
    let mut array = [0; 27];
    responses.trim().split("\n").for_each(|person| {
        person
            .as_bytes()
            .iter()
            .for_each(|answer| array[(answer - b'a') as usize] += 1);
        array[26] += 1
    });
    Ok(array)
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
#[inline(always)]
fn part_1(responses: &[usize; 27]) -> usize {
    responses.iter().take(26).filter(|&c| *c > 0).count()
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
#[inline(always)]
fn part_2(responses: &[usize; 27]) -> usize {
    responses
        .iter()
        .take(26)
        .filter(|&c| *c == responses[26])
        .count()
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
    let buffer: String = std::fs::read_to_string("data/day06.txt").unwrap();

    // Read to object iterator
    let data: Vec<[usize; 27]> = buffer
        .split("\n\n")
        .map(|line| to_array(line).expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find any response
    let start_part_1 = Instant::now();
    let count_1: usize = data.iter().map(|d| part_1(&d)).sum();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching responses
    let start_part_2 = Instant::now();
    let count_2: usize = data.iter().map(|d| part_2(&d)).sum();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let (combined_1, combined_2) = buffer
        .split("\n\n")
        .map(|line| to_array(line).expect("failed to parse line"))
        .fold((0, 0), |acc, responses| {
            (acc.0 + part_1(&responses), acc.1 + part_2(&responses))
        });
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, count_1);
    assert_eq!(combined_2, count_2);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        count_1 as i64,
        count_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(6, "Custom Customs");
    output::print_part(1, "✅ Count", &format!("{}", results.part_1));
    output::print_part(2, "✅ Shared", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
