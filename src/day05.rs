//! Day 5:
//! Two separate passes are required over the data, one to convert from a string to a
//! binary and one to find the valid seat. However, we can still save some time by
//! locating the max seat while filling the binary seat array

use crate::prelude::*;
use itertools::Itertools;

// -----------------------------------------------------------------------------
// Parse FBLR encoded binary
// -----------------------------------------------------------------------------
fn parse_fblr_binary(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |id, c| id * 2 + (*c as usize % 7) % 2) // B, R -> 1; F, L -> 0
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(tuple: (&bool, &bool)) -> bool {
    *tuple.0 && !*tuple.1 // First open seat, so only need to check pair, not triple
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
    let buffer = std::fs::read_to_string("data/day05.txt").unwrap();

    // Read to object iterator
    let data: Vec<usize> = buffer
        .lines()
        .map(|line| parse_fblr_binary(&line))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passports
    let start_part_1 = Instant::now();
    let max_1 = *data.iter().max().unwrap();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passports
    let start_part_2 = Instant::now();
    let mut mask = [false; 2 << 9];
    data.iter().for_each(|&s| mask[s] = true);
    let seat_2 = mask
        .iter()
        .tuple_windows::<(_, _)>()
        .enumerate()
        .find_map(|(i, t)| if part_2(t) { Some(i as i32) } else { None })
        .unwrap()
        + 1;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let mut combined_mask = [false; 2 << 9];
    let mut combined_1 = 0;
    buffer.lines().for_each(|line| {
        let s = parse_fblr_binary(&line);
        combined_mask[s] = true;
        combined_1 = std::cmp::max(combined_1, s);
    });
    let combined_2 = mask
        .iter()
        .tuple_windows::<(_, _)>()
        .enumerate()
        .find_map(|(i, t)| if part_2(t) { Some(i as i32) } else { None })
        .unwrap()
        + 1;
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, max_1);
    assert_eq!(combined_2, seat_2);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        max_1 as i64,
        seat_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(5, "Binary Boarding");
    output::print_part(1, "💺 Largest", &format!("{}", results.part_1));
    output::print_part(2, "💺 Available", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
