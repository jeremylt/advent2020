use crate::prelude::*;

// -----------------------------------------------------------------------------
// Parse line to array
// -----------------------------------------------------------------------------
fn to_array(responses: &str) -> [usize; 27] {
    let mut array = [0; 27];
    responses.trim().split("\n").for_each(|person| {
        person
            .as_bytes()
            .iter()
            .for_each(|answer| array[(answer - b'a') as usize] += 1);
        array[26] += 1
    });
    array
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(responses: &[usize; 27]) -> usize {
    responses[0..26].iter().filter(|&c| *c > 0).count()
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(responses: &[usize; 27]) -> usize {
    responses[0..26]
        .iter()
        .filter(|&c| *c == responses[26])
        .count()
}

// -----------------------------------------------------------------------------
// Day 6
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let buffer: String = std::fs::read_to_string("data/day06.txt").unwrap();

    // Read to object iterator
    let data: Vec<[usize; 27]> = buffer.split("\n\n").map(|line| to_array(line)).collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_1 = Instant::now();
    let count_1: usize = data.iter().map(|d| part_1(&d)).sum();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_2 = Instant::now();
    let count_2: usize = data.iter().map(|d| part_2(&d)).sum();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let (combined_1, combined_2) = buffer
        .split("\n\n")
        .map(|line| to_array(line))
        .fold((0, 0), |acc, responses| {
            (acc.0 + part_1(&responses), acc.1 + part_2(&responses))
        });
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, count_1);
    assert_eq!(combined_2, count_2);

    // -------------------------------------------------------------------------
    // Report
    // -------------------------------------------------------------------------
    if print_summary {
        output::print_day(6);
        output::print_part(1, "✅ Count", &format!("{}", count_1));
        output::print_part(2, "✅ Shared", &format!("{}", count_2));
        output::print_timing(Timing {
            setup: time_setup,
            part_1: time_part_1,
            part_2: time_part_2,
            combined: time_combined,
        });
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part_1: count_1 as i64,
        part_2: count_2 as i64,
        time: time_combined,
    };
}

// -----------------------------------------------------------------------------
