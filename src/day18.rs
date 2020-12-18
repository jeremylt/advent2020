//! Day 18:
//! Part 1 was a straightforward right-to-left traversal of the lines. For Part 2
//! I executed all + in the right-to-left traversal and then took the product of
//! the resulting sums. I was able to take some shortcuts based on my knowledge
//! of the input structure that I would not otherwise be able to do, like knowing the
//! numbers are all one character long and knowing the distribution of the spaces.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(s: &str) -> usize {
    let length = s.len();
    // Right side
    let right: usize;
    let operation_index: usize;
    if s.as_bytes()[length - 1] == b')' {
        let mut count = 1;
        operation_index = s
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .skip(1)
            .find_map(|(i, &c)| {
                if c == b')' {
                    count += 1;
                } else if c == b'(' {
                    count -= 1;
                }
                if count == 0 {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap()
            + 3;
        right = part_1(&s[length - operation_index + 3..length - 1]);
    } else {
        right = (s.as_bytes()[length - 1] - b'0') as usize;
        operation_index = 3;
    }
    if operation_index > length {
        return right;
    }
    // Left side
    let left: usize = part_1(&s[0..length - operation_index - 1]);
    // Operation
    if s.as_bytes()[length - operation_index] == b'+' {
        left + right
    } else {
        left * right
    }
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn evaluate_next(s: &str) -> (usize, usize) {
    let length = s.len();
    // Right side
    let right: usize;
    let operation_index: usize;
    if s.as_bytes()[length - 1] == b')' {
        let mut count = 1;
        operation_index = s
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .skip(1)
            .find_map(|(i, &c)| {
                if c == b')' {
                    count += 1;
                } else if c == b'(' {
                    count -= 1;
                }
                if count == 0 {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap()
            + 3;
        right = part_2(&s[length - operation_index + 3..length - 1]);
    } else {
        right = (s.as_bytes()[length - 1] - b'0') as usize;
        operation_index = 3;
    }
    (right, operation_index)
}

fn part_2(s: &str) -> usize {
    let length = s.len();
    let mut current_index = length;
    let mut current_sum = 0;
    let mut sums = vec![];
    // Eagerly evaluate sums, then evaluate products, respecting ()s
    while current_index > 0 {
        let (right, operation_index) = evaluate_next(&s[0..current_index]);
        current_index = current_index.saturating_sub(operation_index + 1);
        current_sum += right;
        if current_index == 0 || s.as_bytes()[current_index + 1] == b'*' {
            sums.push(current_sum);
            current_sum = 0;
        }
    }
    sums.iter().product()
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
    let buffer: String = std::fs::read_to_string("data/day18.txt").unwrap();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Parse 'new math'
    let start_part_1 = Instant::now();
    let calculation_1: usize = buffer.lines().map(|line| part_1(line)).sum();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Parse advanced 'new math'
    let start_part_2 = Instant::now();
    let calculation_2: usize = buffer.lines().map(|line| part_2(line)).sum();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let (combined_1, combined_2) = buffer
        .lines()
        .map(|line| (part_1(line), part_2(line)))
        .fold((0, 0), |acc, values| (acc.0 + values.0, acc.1 + values.1));
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, calculation_1);
    assert_eq!(combined_2, calculation_2);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        calculation_1 as i64,
        calculation_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(18, "Operation Order");
    output::print_part(1, "🧮 Calculation", &format!("{}", results.part_1));
    output::print_part(2, "🧮 Calculation", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
