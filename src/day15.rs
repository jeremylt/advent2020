//! Day 15:
//! Nothing clever here.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Play game
// -----------------------------------------------------------------------------
#[inline]
fn play_game(n: usize, starters: &Vec<usize>) -> u32 {
    let mut said = vec![u32::MAX; n + 1];
    let number_starters = starters.len();
    starters
        .iter()
        .take(number_starters - 1)
        .enumerate()
        .for_each(|(i, &value)| {
            said[value] = (i + 1) as u32;
        });

    (number_starters..n).fold(starters[number_starters - 1] as u32, |current, i| {
        (i as u32).saturating_sub(std::mem::replace(&mut said[current as usize], i as u32))
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
    let number_1 = play_game(2020, &values);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find 30000000th number
    let start_part_2 = Instant::now();
    let number_2 = play_game(30_000_000, &values);
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
    output::print_day(15, "Rambunctious Recitation");
    output::print_part(1, "🧝 Number", &format!("{}", results.part_1));
    output::print_part(2, "🧝 Number", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
