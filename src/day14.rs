//! Day 14:
//! The bitwise operations are straightforward, but the combinatorics make this tricky.
//! 64 bit integers are slower to work with, and my current solution to Part 2 is
//! slow in hitting all required floating mask values.

use crate::prelude::*;
use arrayvec::ArrayVec;
use rustc_hash::FxHashSet;

// Constants
const CAPACITY: usize = 4096;
const INSTRUCTIONS: usize = 8;
const MAX_36_BITS: u64 = u64::max_value() >> (64 - 36);

// -----------------------------------------------------------------------------
// Instructions
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Instructions {
    set_mask: u64,
    clear_mask: u64,
    floating_mask: u64,
    updates: ArrayVec<[Update; INSTRUCTIONS]>,
}

impl std::str::FromStr for Instructions {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.lines();
        // Masks
        let mut set_mask = 0;
        let mut clear_mask = 0;
        data.next()
            .unwrap()
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, &b)| {
                if b == b'1' {
                    set_mask |= 1 << (35 - i);
                } else if b == b'0' {
                    clear_mask |= 1 << (35 - i);
                }
            });
        clear_mask = !clear_mask;
        let floating_mask = set_mask | !clear_mask;
        // Updates
        let updates = data
            .map(|line| line.parse::<Update>().expect("failed to parse update"))
            .collect();
        // Return
        Ok(Self {
            set_mask,
            clear_mask,
            floating_mask,
            updates,
        })
    }
}

#[derive(Debug)]
struct Update {
    address: u64,
    value: u64,
}

impl std::str::FromStr for Update {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.splitn(2, "]");
        let address: u64 = data.next().unwrap()[4..].parse().unwrap();
        let value: u64 = data.next().unwrap()[3..].parse().unwrap();
        Ok(Self { address, value })
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
#[inline]
fn update_memory_1(instructions: &Instructions, memory: &mut FxHashSet<u64>) -> u64 {
    let mut sum = 0;
    instructions.updates.iter().rev().for_each(|update| {
        if memory.insert(update.address) {
            sum += (update.value | instructions.set_mask) & instructions.clear_mask;
        }
    });
    sum
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
#[inline]
fn update_memory_2(instructions: &Instructions, memory: &mut FxHashSet<u64>) -> u64 {
    let mut sum = 0;
    instructions.updates.iter().rev().for_each(|update| {
        let base_address: u64 =
            (update.address | instructions.set_mask) & instructions.floating_mask;

        let mut current_mask = instructions.floating_mask;
        loop {
            if memory.insert(base_address | (!current_mask & MAX_36_BITS)) {
                sum += update.value;
            }
            if current_mask & MAX_36_BITS == MAX_36_BITS {
                break;
            }
            current_mask = (current_mask + 1) | instructions.floating_mask;
        }
    });
    sum
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
    let buffer: String = std::fs::read_to_string("data/day14.txt").unwrap();

    // Read to object iterator
    let data: Vec<Instructions> = buffer
        .split("mask = ")
        .skip(1)
        .map(|line| {
            line.parse::<Instructions>()
                .expect("failed to parse instructions")
        })
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Apply value bitmasks
    let start_part_1 = Instant::now();
    let mut memory = FxHashSet::<u64>::with_capacity_and_hasher(CAPACITY, Default::default());
    let sum_1 = data
        .iter()
        .rev()
        .map(|instructions| update_memory_1(instructions, &mut memory))
        .sum::<u64>();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Apply memory bitmasks
    let start_part_2 = Instant::now();
    memory.clear();
    let sum_2 = data
        .iter()
        .rev()
        .map(|instructions| update_memory_2(instructions, &mut memory))
        .sum::<u64>();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        sum_1 as i64,
        sum_2 as i64,
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
    output::print_day(14, "Docking Data");
    output::print_part(1, "🏗 Sum", &format!("{}", results.part_1));
    output::print_part(2, "🏗 Sum", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
