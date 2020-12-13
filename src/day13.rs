//! Day 13:
//! This is a fast one so long as you use the Chinese Remainder Theorem and modular
//! arithmetic. Nothing particularly noteworthy in the code.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Bus object
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Bus {
    id: usize,
    time: usize,
}

// -----------------------------------------------------------------------------
// Extended Euclidean algorithm
// -----------------------------------------------------------------------------
fn euclid_inverse(mut a: i64, mut b: i64) -> usize {
    if b == 1 {
        return 1;
    }
    let b0 = b;
    let mut t;
    let mut q;
    let mut x0 = 0;
    let mut x1 = 1;
    while a > 1 {
        q = a / b;
        t = b;
        b = a % b;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 {
        x1 += b0;
    }
    x1 as usize
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
    let buffer: String = std::fs::read_to_string("data/day13.txt").unwrap();

    // Read to vector
    let earliest: usize = buffer
        .lines()
        .nth(0)
        .unwrap()
        .parse()
        .expect("failed to parse line");
    let values: Vec<Bus> = buffer
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, entry)| *entry != "x")
        .map(|(i, entry)| {
            let value = entry.parse().expect("failed to parse line");
            Bus {
                id: value,
                time: i % value,
            }
        })
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find first bus
    let start_part_1 = Instant::now();
    let (bus_1, minutes_1) = values
        .iter()
        .map(|bus| (bus.id, bus.id - (earliest % bus.id)))
        .fold((earliest, earliest), |acc, time| {
            if time.1 < acc.1 {
                time
            } else {
                acc
            }
        });
    let product_1 = bus_1 * minutes_1;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find bus sequence
    let start_part_2 = Instant::now();
    let m = values.iter().fold(1, |acc, bus| acc * bus.id);
    let departure_2: usize = values
        .iter()
        .skip(1)
        .map(|bus| {
            let m_i = m / bus.id;
            ((bus.id - bus.time) * euclid_inverse(m_i as i64, bus.id as i64) * m_i) % m
        })
        .sum::<usize>()
        % m;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        product_1 as i64,
        departure_2 as i64,
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
    output::print_day(13, "Shuttle Search");
    output::print_part(1, "🚌 Product", &format!("{}", results.part_1));
    output::print_part(2, "🚌 Sequence", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
