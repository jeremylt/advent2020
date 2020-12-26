//! Day 25:
//! For this puzzle I used the baby step, giant step algorithm to compute
//! the discrete logarithm. With a known base and modulus, I hard-coded the
//! prerequsite values for the algorithm. I use squares to efficiently apply
//! the secret key to derive the shared secret.

use crate::prelude::*;
use rustc_hash::FxHashMap;

// Constant
const BASE: u32 = 7;
const P: u32 = 2020_12_27;
const SQRT_P: u32 = 4_495;
const BASE_INV_SQRT_P: u32 = 680_915;

// -----------------------------------------------------------------------------
// Discrete logarithm
// -----------------------------------------------------------------------------
// Baby step, giant step algorithm
fn discrete_log(target: u32) -> u32 {
    // Form table of a^j mod p
    let mut current = 1;
    let table: FxHashMap<u32, u32> = (0..SQRT_P)
        .map(|j| {
            let previous = current;
            current = ((current as u64 * BASE as u64) % P as u64) as u32;
            (previous, j)
        })
        .collect();

    // Find logarithm
    let mut current = target;
    for i in 0..SQRT_P {
        if table.contains_key(&current) {
            return i * SQRT_P + table.get(&current).unwrap();
        } else {
            current = ((current as u64 * BASE_INV_SQRT_P as u64) % P as u64) as u32;
        }
    }
    unreachable!()
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
    let buffer: String = std::fs::read_to_string("data/day25.txt").unwrap();

    // Read data
    let mut keys = buffer
        .lines()
        .map(|line| line.parse::<u32>().expect("failed to parse key"));
    let door_public_key = keys.next().unwrap();
    let card_public_key = keys.next().unwrap();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Discover shared secret
    let start_part_1 = Instant::now();
    let door_private_key = discrete_log(door_public_key);
    let number_digits = (door_private_key as f32).log2().ceil() as usize;
    let mut digits = door_private_key;
    let mut square = card_public_key;
    // Multiply result by powers of two found in binary representation of loop size
    let secret_1 = (0..number_digits).fold(1, |acc, _| {
        let new_result = if digits % 2 == 1 {
            ((acc as u64 * square as u64) % P as u64) as u32
        } else {
            acc
        };
        square = ((square as u64 * square as u64) % P as u64) as u32;
        digits = digits >> 1;
        new_result
    });
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // No part 2
    let start_part_2 = Instant::now();
    let count_2 = 0;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        secret_1 as i64,
        count_2 as i64,
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
    output::print_day(25, "Combo Breaker");
    output::print_part(1, "🔓 Secret", &format!("{}", results.part_1));
    output::print_part(2, "🔓 HAPPY HOLIDAYS", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
