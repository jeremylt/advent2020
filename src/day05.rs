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
fn part_2(triple: (&bool, &bool, &bool)) -> bool {
    *triple.0 && !*triple.1 && *triple.2
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
        .tuple_windows::<(_, _, _)>()
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
    buffer
        .lines()
        .map(|line| parse_fblr_binary(&line))
        .for_each(|s| {
            combined_mask[s] = true;
            combined_1 = std::cmp::max(combined_1, s)
        });
    let combined_2 = mask
        .iter()
        .tuple_windows::<(_, _, _)>()
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
    return Results {
        part_1: max_1 as i64,
        part_2: seat_2 as i64,
        times: Timing {
            setup: time_setup,
            part_1: time_part_1,
            part_2: time_part_2,
            combined: time_combined,
        },
    };
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(5);
    output::print_part(1, "💺 Largest", &format!("{}", results.part_1));
    output::print_part(2, "💺 Available", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
