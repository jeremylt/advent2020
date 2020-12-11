//! Day 11:

use crate::prelude::*;
use arrayvec::ArrayVec;

// Constants
const NEIGHBORS: usize = 8;

// -----------------------------------------------------------------------------
// Game of Life
// -----------------------------------------------------------------------------
#[inline(always)]
fn game_of_life(
    seats: &mut Vec<u8>,
    check_seats: &Vec<usize>,
    max_neighbors: usize,
    check_neighbors: &Vec<ArrayVec<[usize; NEIGHBORS]>>,
) {
    let mut changed: Vec<usize> = vec![];
    let mut repeat = true;
    while repeat {
        check_seats.iter().for_each(|&i| {
            // Check seats for change
            if seats[i] == 0 {
                // Empty seat
                if check_neighbors[i].iter().all(|&index| seats[index] != 1) {
                    changed.push(i);
                }
            } else if check_neighbors[i].len() >= max_neighbors {
                // Occupied seat
                if check_neighbors[i]
                    .iter()
                    .filter(|&index| seats[*index] == 1)
                    .count()
                    >= max_neighbors
                {
                    changed.push(i);
                }
            }
        });
        if changed.len() > 0 {
            // Apply changes
            changed
                .iter()
                .for_each(|&index| seats[index] = (seats[index] + 1) % 2);
            changed.clear();
        } else {
            // Exit simulation
            repeat = false;
        }
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(
    seats: &Vec<u8>,
    row_length: usize,
    number_rows: usize,
) -> Vec<ArrayVec<[usize; NEIGHBORS]>> {
    seats
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut indices = ArrayVec::new();
            let row = i / row_length;
            let col = i % row_length;
            if col > 0 {
                let index = i - 1;
                if seats[index] < 2 {
                    indices.push(index);
                }
                if row > 0 {
                    if seats[index - row_length] < 2 {
                        indices.push(index - row_length);
                    }
                }
            }
            if col < row_length - 1 {
                let index = i + 1;
                if seats[index] < 2 {
                    indices.push(index);
                }
                if row < number_rows - 1 {
                    if seats[index + row_length] < 2 {
                        indices.push(index + row_length);
                    }
                }
            }
            if row > 0 {
                let index = i - row_length;
                if seats[index] < 2 {
                    indices.push(index);
                }
                if col < row_length - 1 {
                    if seats[index + 1] < 2 {
                        indices.push(index + 1);
                    }
                }
            }
            if row < number_rows - 1 {
                let index = i + row_length;
                if seats[index] < 2 {
                    indices.push(index);
                }
                if col > 0 {
                    if seats[index - 1] < 2 {
                        indices.push(index - 1);
                    }
                }
            }
            indices
        })
        .collect::<Vec<ArrayVec<[usize; NEIGHBORS]>>>()
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(
    seats: &Vec<u8>,
    row_length: usize,
    number_rows: usize,
) -> Vec<ArrayVec<[usize; NEIGHBORS]>> {
    seats
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut indices = ArrayVec::new();
            let row = i / row_length;
            let col = i % row_length;
            if col > 0 {
                let mut left = 1;
                while left <= col && seats[i - left] == 2 {
                    left += 1;
                }
                if left <= col {
                    indices.push(i - left);
                }
                if row > 0 {
                    let mut diag = 1;
                    while diag <= col && diag <= row && seats[i - diag - diag * row_length] == 2 {
                        diag += 1;
                    }
                    if diag <= col && diag <= row {
                        indices.push(i - diag - diag * row_length);
                    }
                }
            }
            if col < row_length - 1 {
                let mut right = 1;
                while right <= row_length - 1 - col && seats[i + right] == 2 {
                    right += 1;
                }
                if right <= row_length - 1 - col {
                    indices.push(i + right);
                }
                if row < number_rows - 1 {
                    let mut diag = 1;
                    while diag <= row_length - 1 - col
                        && diag <= number_rows - 1 - row
                        && seats[i + diag + diag * row_length] == 2
                    {
                        diag += 1;
                    }
                    if diag <= row_length - 1 - col && diag <= number_rows - 1 - row {
                        indices.push(i + diag + diag * row_length);
                    }
                }
            }
            if row > 0 {
                let mut up = 1;
                while up <= row && seats[i - up * row_length] == 2 {
                    up += 1;
                }
                if up <= row {
                    indices.push(i - up * row_length);
                }
                if col < row_length - 1 {
                    let mut diag = 1;
                    while diag <= row
                        && diag <= row_length - 1 - col
                        && seats[i + diag - diag * row_length] == 2
                    {
                        diag += 1;
                    }
                    if diag <= row && diag <= row_length - 1 - col {
                        indices.push(i + diag - diag * row_length);
                    }
                }
            }
            if row < number_rows - 1 {
                let mut down = 1;
                while down <= number_rows - 1 - row && seats[i + down * row_length] == 2 {
                    down += 1;
                }
                if down <= number_rows - 1 - row {
                    indices.push(i + down * row_length);
                }
                if col > 0 {
                    let mut diag = 1;
                    while diag <= col
                        && diag <= number_rows - 1 - row
                        && seats[i - diag + diag * row_length] == 2
                    {
                        diag += 1;
                    }
                    if diag <= col && diag <= number_rows - 1 - row {
                        indices.push(i - diag + diag * row_length);
                    }
                }
            }
            indices
        })
        .collect::<Vec<ArrayVec<[usize; NEIGHBORS]>>>()
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
    let row_length = buffer.lines().nth(0).unwrap().chars().count();
    let mut seats: Vec<u8> = buffer
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| if c == 'L' { 1 } else { 2 })
        .collect();
    let number_rows = seats.len() / row_length;
    let check_seats = seats
        .iter()
        .enumerate()
        .filter_map(|(i, &value)| if value != 2 { Some(i) } else { None })
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find stable configuration
    let start_part_1 = Instant::now();

    // Neighbors to check
    let check_neighbors = part_1(&seats, row_length, number_rows);

    // Run Game of Life
    game_of_life(&mut seats, &check_seats, 4, &check_neighbors);
    let count_1 = seats.iter().filter(|&s| *s == 1).count();

    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Revised seat rules
    let start_part_2 = Instant::now();

    // Reset
    seats.iter_mut().for_each(|value| {
        if *value == 0 {
            *value = 1;
        }
    });

    // Neighbors to check
    let check_neighbors = part_2(&seats, row_length, number_rows);

    // Run Game of Life
    game_of_life(&mut seats, &check_seats, 5, &check_neighbors);
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
    output::print_day(1, "Report Repair");
    output::print_part(1, "⛴ Occupied", &format!("{}", results.part_1));
    output::print_part(2, "⛴ Occupied", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
