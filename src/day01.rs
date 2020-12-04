use crate::prelude::*;

// Constant
const YEAR: usize = 2020;

// -----------------------------------------------------------------------------
// Find pair of flaged indices that sum to length of mask array
// -----------------------------------------------------------------------------
fn find_two(array: &[bool]) -> Option<i32> {
    let value = array
        .iter()
        .enumerate()
        .zip(array.iter().rev())
        .find_map(|((i, a), b)| if a & b { Some(i as i32) } else { None });
    value
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(mask: &[bool]) -> (i32, i32) {
    match find_two(&mask) {
        Some(index) => (index, YEAR as i32 - index),
        None => panic!("No pair found"),
    }
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(values: &[usize], mask: &[bool]) -> (i32, i32, i32) {
    for value in values {
        let remainder = YEAR - *value;
        let index = find_two(&mask[0..=remainder]);
        if index != None {
            let triple = (
                *value as i32,
                index.unwrap(),
                YEAR as i32 - *value as i32 - index.unwrap(),
            );
            return triple;
        }
    }
    panic!("No triple found");
}

// -----------------------------------------------------------------------------
// Day 1
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    if print_summary {
        output::print_day(1);
    }
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let buffer = file::load_file("data/day01part01.txt");

    // Read to vector
    let values: Vec<usize> = buffer
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Mask array
    let mut mask = [false; YEAR + 1];
    for &value in &values {
        mask[value] = true;
    }

    // Time
    let time_setup = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Look for pair
    let start_part_1 = Instant::now();
    let tuple = part_1(&mask);
    let product_1 = tuple.0 * tuple.1;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let start_part_2 = Instant::now();
    let triple = part_2(&values, &mask);
    let product_2 = triple.0 * triple.1 * triple.2;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Timing
    // -------------------------------------------------------------------------
    let time = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Report
    // -------------------------------------------------------------------------
    if print_summary {
        output::print_setup(time_setup);
        output::print_part(
            1,
            "Values",
            &format!("{:?}", tuple),
            "📄 Product",
            &format!("{}", product_1),
            time_part_1,
        );
        output::print_part(
            2,
            "Values",
            &format!("{:?}", triple),
            "📄 Product",
            &format!("{}", product_2),
            time_part_2,
        );
        output::print_timing(time, time_part_1, time_part_2);
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part_1: product_1 as i64,
        part_2: product_2 as i64,
        time: time.as_nanos(),
    };
}

// -----------------------------------------------------------------------------
