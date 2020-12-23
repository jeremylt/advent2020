//! Day 23:
//! Another "do the same thing millions of times" challenge. Using arrays instead of
//! vectors would be easier, but that large of arrays can overflow the stack for the
//! test suite.

use crate::prelude::*;

// Constants
const NUMBER_CUPS_SMALL: i32 = 9;
const NUMBER_ROUNDS_SMALL: usize = 100;
const NUMBER_CUPS_BIG: i32 = 1_000_000;
const NUMBER_ROUNDS_BIG: usize = 10_000_000;

// -----------------------------------------------------------------------------
// Play the game
// -----------------------------------------------------------------------------
#[inline(always)]
fn play_game(cups: &mut Vec<i32>, number_rounds: usize, number_cups: i32) {
    let mut current_cup: i32 = 0;
    (0..number_rounds).for_each(|_| {
        // Next 3 cups
        let next_1 = cups[current_cup as usize];
        let next_2 = cups[next_1 as usize];
        let next_3 = cups[next_2 as usize];

        // Get target
        let mut target_cup = (current_cup - 1).rem_euclid(number_cups);
        while [next_1, next_2, next_3].contains(&target_cup) {
            target_cup = (target_cup - 1).rem_euclid(number_cups);
        }

        // Update indices
        cups[current_cup as usize] = cups[next_3 as usize];
        cups[next_3 as usize] = cups[target_cup as usize];
        cups[target_cup as usize] = next_1;

        // Increment current
        current_cup = cups[current_cup as usize];
    });
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
    let buffer: String = std::fs::read_to_string("data/day23.txt").unwrap();

    // Read to vector
    let mut cups = vec![0; NUMBER_CUPS_SMALL as usize];
    let data: Vec<i32> = buffer
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32 - 1)
        .collect();
    (0..NUMBER_CUPS_SMALL as usize)
        .for_each(|i| cups[data[i] as usize] = data[(i + 1) % NUMBER_CUPS_SMALL as usize]);
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find 100th move
    let start_part_1 = Instant::now();
    play_game(&mut cups, NUMBER_ROUNDS_SMALL, NUMBER_CUPS_SMALL);

    let mut current_index = cups[0];
    let number_1 = (0..NUMBER_CUPS_SMALL - 1).fold(0, |acc, _| {
        let new = (acc * 10) + (current_index + 1);
        current_index = cups[current_index as usize];
        new
    });
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find 10,000,000th move
    let start_part_2 = Instant::now();
    let mut more_cups = vec![0; NUMBER_CUPS_BIG as usize];
    (0..NUMBER_CUPS_SMALL as usize - 1)
        .for_each(|i| more_cups[data[i] as usize] = data[(i + 1) % NUMBER_CUPS_SMALL as usize]);
    more_cups[data[NUMBER_CUPS_SMALL as usize - 1] as usize] = NUMBER_CUPS_SMALL;
    (NUMBER_CUPS_SMALL as usize..NUMBER_CUPS_BIG as usize)
        .for_each(|i| more_cups[i] = (i as i32 + 1) % NUMBER_CUPS_BIG);

    play_game(&mut more_cups, NUMBER_ROUNDS_BIG, NUMBER_CUPS_BIG);

    let number_2 = (more_cups[0] as usize + 1) * (more_cups[more_cups[0] as usize] as usize + 1);
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
    output::print_day(23, "Crab Cups");
    output::print_part(1, "🥛 Current", &format!("{}", results.part_1));
    output::print_part(2, "🥛 Number", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
