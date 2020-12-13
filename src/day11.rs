//! Day 11:
//! This puzzle is similar to Conway's Game of Life. As such, I have taken some hints
//! from optimized Game of Life simulations. Specifically, I precompute the indices to
//! check for each day, and I maintain a reducing list of seats to recheck. Of note,
//! I discovered that retain is faster than filter if you are keeping the vector modified.

use crate::prelude::*;
use arrayvec::ArrayVec;

// Constants
const CAPACITY: usize = 16384;

// -----------------------------------------------------------------------------
// Game of Life
// -----------------------------------------------------------------------------
#[inline]
fn game_of_life(
    seats: &mut ArrayVec<[u8; CAPACITY]>,
    check_seats: &mut Vec<u16>,
    max_neighbors: u8,
    check_neighbors: &ArrayVec<[[u16; 8]; CAPACITY]>,
) {
    let mut changed: ArrayVec<[u16; CAPACITY]> = ArrayVec::new();
    let mut repeat = true;
    while repeat {
        check_seats.retain(|&i| {
            // Check seats for change
            let count = check_neighbors[i as usize]
                .iter()
                .map(|&index| (seats[index as usize] % 2))
                .sum::<u8>();
            if (seats[i as usize] == 0 && count == 0)
                || (seats[i as usize] == 1 && count >= max_neighbors)
            {
                changed.push(i);
                true
            } else {
                false
            }
        });
        repeat = false;
        // Apply changes
        changed.iter().for_each(|&index| {
            repeat = true;
            seats[index as usize] = (seats[index as usize] + 1) % 2;
        });
        changed.clear();
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
#[inline]
fn part_1(row_length: u16, number_rows: u16) -> ArrayVec<[[u16; 8]; CAPACITY]> {
    (0..number_rows - 1)
        .flat_map(|i| {
            if i == 0 {
                vec![[0; 8]; row_length as usize]
            } else {
                (0..row_length)
                    .map(|j| {
                        if j == 0 || j == row_length - 1 {
                            [0; 8]
                        } else {
                            let index = i * row_length + j;
                            [
                                index - row_length - 1,
                                index - row_length,
                                index - row_length + 1,
                                index - 1,
                                index + 1,
                                index + row_length - 1,
                                index + row_length,
                                index + row_length + 1,
                            ]
                        }
                    })
                    .collect()
            }
        })
        .collect()
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
#[inline]
fn part_2(
    seats: &ArrayVec<[u8; CAPACITY]>,
    row_length: u16,
    number_rows: u16,
) -> ArrayVec<[[u16; 8]; CAPACITY]> {
    (0..number_rows - 1)
        .flat_map(|i| {
            if i == 0 {
                vec![[0; 8]; row_length as usize]
            } else {
                (0..row_length)
                    .map(|j| {
                        if j == 0 || j == row_length - 1 {
                            [0; 8]
                        } else {
                            let index = i * row_length + j;
                            let mut left_up = index - row_length - 1;
                            while seats[left_up as usize] == 2 {
                                left_up -= row_length + 1;
                            }
                            let mut up = index - row_length;
                            while seats[up as usize] == 2 {
                                up -= row_length;
                            }
                            let mut right_up = index - row_length + 1;
                            while seats[right_up as usize] == 2 {
                                right_up -= row_length - 1;
                            }
                            let mut left = index - 1;
                            while seats[left as usize] == 2 {
                                left -= 1;
                            }
                            let mut right = index + 1;
                            while seats[right as usize] == 2 {
                                right += 1;
                            }
                            let mut left_down = index + row_length - 1;
                            while seats[left_down as usize] == 2 {
                                left_down += row_length - 1;
                            }
                            let mut down = index + row_length;
                            while seats[down as usize] == 2 {
                                down += row_length;
                            }
                            let mut right_down = index + row_length + 1;
                            while seats[right_down as usize] == 2 {
                                right_down += row_length + 1;
                            }
                            [
                                left_up, up, right_up, left, right, left_down, down, right_down,
                            ]
                        }
                    })
                    .collect()
            }
        })
        .collect()
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
    let buffer: String = std::fs::read_to_string("data/day11.txt").unwrap();

    // Read to vector
    let row_length = buffer.lines().nth(0).unwrap().chars().count() as u16 + 2;
    let number_rows = buffer.lines().count() as u16 + 2;
    let mut seats: ArrayVec<[u8; CAPACITY]> = ArrayVec::from([0; CAPACITY]);
    buffer.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            seats[(i + 1) * row_length as usize + j + 1] = if c == 'L' { 1 } else { 2 }
        })
    });
    let mut check_seats = seats
        .iter()
        .enumerate()
        .filter_map(|(i, &value)| if value == 1 { Some(i as u16) } else { None })
        .collect::<Vec<u16>>();

    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find stable configuration
    let start_part_1 = Instant::now();

    // Neighbors to check
    let check_neighbors = part_1(row_length, number_rows);

    // Run Game of Life
    game_of_life(&mut seats, &mut check_seats, 4, &check_neighbors);
    let count_1 = seats.iter().filter(|&s| *s == 1).count();

    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Revised seat rules
    let start_part_2 = Instant::now();

    // Reset
    (1..number_rows as usize - 1).for_each(|i| {
        (1..row_length as usize - 1).for_each(|j| {
            if seats[i * row_length as usize + j] == 0 {
                seats[i * row_length as usize + j] = 1;
            }
        })
    });
    let mut check_seats = seats
        .iter()
        .enumerate()
        .filter_map(|(i, &value)| if value == 1 { Some(i as u16) } else { None })
        .collect::<Vec<u16>>();

    // Neighbors to check
    let check_neighbors = part_2(&seats, row_length, number_rows);

    // Run Game of Life
    game_of_life(&mut seats, &mut check_seats, 5, &check_neighbors);
    let count_2 = seats.iter().filter(|&s| *s == 1).count();

    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        count_1 as i64,
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
    output::print_day(11, "Seating System");
    output::print_part(1, "⛴ Occupied", &format!("{}", results.part_1));
    output::print_part(2, "⛴ Occupied", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
