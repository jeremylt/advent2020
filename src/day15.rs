//! Day 15:
//! Nothing clever here. I am using a vec for the dense portion of the integers and a
//! hash map for the sparse portion of the integers.

use crate::prelude::*;
use rustc_hash::FxHashMap;

// Constants
const BREAKPOINT: usize = 1 << 22;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
#[inline]
fn part_1(n: usize, starters: &Vec<usize>) -> u32 {
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
// Part 2
// -----------------------------------------------------------------------------
#[inline]
fn part_2(n: usize, starters: &Vec<usize>) -> u32 {
    let mut said = vec![u32::MAX; BREAKPOINT];
    let mut said_big =
        FxHashMap::<u32, u32>::with_capacity_and_hasher(BREAKPOINT / 64, Default::default());
    let number_starters = starters.len();
    starters
        .iter()
        .take(number_starters - 1)
        .enumerate()
        .for_each(|(i, &value)| {
            said[value] = (i + 1) as u32;
        });

    // Lower portion of range
    let lower =
        (number_starters..BREAKPOINT).fold(starters[number_starters - 1] as u32, |current, i| {
            (i as u32).saturating_sub(std::mem::replace(&mut said[current as usize], i as u32))
        });
    // Upper portion of range
    (BREAKPOINT..n).fold(lower, |current, i| {
        if current < BREAKPOINT as u32 {
            (i as u32).saturating_sub(std::mem::replace(&mut said[current as usize], i as u32))
        } else {
            match said_big.insert(current, i as u32) {
                None => 0,
                Some(value) => i as u32 - value,
            }
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
    let number_1 = part_1(2020, &values);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find 30000000th number
    let start_part_2 = Instant::now();
    let number_2 = part_2(30_000_000, &values);
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
