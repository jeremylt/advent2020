//! Day 3:
//! An important observation from today is that you get better performance from a combined
//! map and fold/for_each that you get from chaining the two.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Count trees
// -----------------------------------------------------------------------------
#[inline(always)]
fn hit_tree(line: &String, i: &usize, right: usize, down: usize, line_length: usize) -> bool {
    // Note: this probably incurs bounds checking, but this day is already fast
    (i % down == 0) && (line.as_bytes()[right * i / down % line_length] == b'#')
}

#[inline(always)]
fn count_trees(data: &Vec<String>, right: usize, down: usize, line_length: usize) -> usize {
    data.iter()
        .enumerate()
        .filter(|(i, line)| hit_tree(*line, i, right, down, line_length))
        .count()
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
    let buffer: String = std::fs::read_to_string("data/day03.txt").unwrap();

    // Read to object iterator
    let data: Vec<String> = buffer.lines().map(|line| line.to_string()).collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find trees on path
    let start_part_1 = Instant::now();
    let line_length = data[0].chars().count();
    let count_1 = count_trees(&data, 3, 1, line_length);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find product of trees on paths
    let start_part_2 = Instant::now();
    let slopes = [(1, 1), (5, 1), (7, 1), (1, 2)];
    let product_2 = slopes.iter().fold(count_1, |product, &(right, down)| {
        product * count_trees(&data, right, down, line_length)
    });
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let slopes = [(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];
    let line_length = buffer.lines().nth(0).unwrap().chars().count();
    let mut trees_hit = [0; 5];
    buffer.lines().enumerate().for_each(|(i, line)| {
        let data = line.to_string();
        slopes.iter().enumerate().for_each(|(j, &(right, down))| {
            trees_hit[j] += hit_tree(&data, &i, right, down, line_length) as usize
        });
    });
    let combined_1 = trees_hit[0];
    let combined_2: usize = trees_hit.iter().product();
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, count_1);
    assert_eq!(combined_2, product_2);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        count_1 as i64,
        product_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(3, "Toboggan Trajectory");
    output::print_part(1, "🌲 Count", &format!("{}", results.part_1));
    output::print_part(2, "🌲 Product", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
