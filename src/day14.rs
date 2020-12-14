//! Day 14:

use crate::prelude::*;
use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;

// Constants
const CAPACITY: usize = 512;
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
fn update_memory_1(instructions: &Instructions, memory: &mut FxHashMap<u64, u64>) {
    instructions.updates.iter().for_each(|update| {
        memory.insert(
            update.address,
            (update.value | instructions.set_mask) & instructions.clear_mask,
        );
    });
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
#[inline]
fn update_memory_2(instructions: &Instructions, memory: &mut FxHashMap<u64, u64>) {
    instructions.updates.iter().for_each(|update| {
        let base_address: u64 =
            (update.address | instructions.set_mask) & instructions.floating_mask;

        let mut current_mask = instructions.floating_mask;
        loop {
            memory.insert(base_address | (!current_mask & MAX_36_BITS), update.value);
            if current_mask & MAX_36_BITS == MAX_36_BITS {
                break;
            }
            current_mask += 1;
            current_mask |= instructions.floating_mask;
        }
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
    let mut memory = FxHashMap::<u64, u64>::with_capacity_and_hasher(CAPACITY, Default::default());
    data.iter()
        .for_each(|instructions| update_memory_1(instructions, &mut memory));
    let sum_1 = memory.iter().fold(0, |acc, (_, value)| acc + value);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Apply memory bitmasks
    let start_part_2 = Instant::now();
    memory.clear();
    data.iter()
        .for_each(|instructions| update_memory_2(instructions, &mut memory));
    let sum_2 = memory.iter().fold(0, |acc, (_, value)| acc + value);
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let mut memory_1 =
        FxHashMap::<u64, u64>::with_capacity_and_hasher(CAPACITY, Default::default());
    let mut memory_2 =
        FxHashMap::<u64, u64>::with_capacity_and_hasher(CAPACITY, Default::default());
    data.iter().for_each(|instructions| {
        update_memory_1(instructions, &mut memory_1);
        update_memory_2(instructions, &mut memory_2);
    });
    let combined_1 = memory_1.iter().fold(0, |acc, (_, value)| acc + value);
    let combined_2 = memory_2.iter().fold(0, |acc, (_, value)| acc + value);
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, sum_1);
    assert_eq!(combined_2, sum_2);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        sum_1 as i64,
        sum_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(1, "Docking Data");
    output::print_part(1, "🏗 Sum", &format!("{}", results.part_1));
    output::print_part(2, "🏗 Sum", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
