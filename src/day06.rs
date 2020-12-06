use crate::prelude::*;
use std::collections::HashSet;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(responses: &str) -> usize {
    let mut uniques = HashSet::new();
    let mut count = 0;
    responses.as_bytes().iter().for_each(|&c| {
        if c != b'\n' {
            count += uniques.insert(c.clone()) as usize;
        }
    });
    count
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(responses: &str) -> usize {
    let mut people = responses.trim().split("\n");
    let mut first = people.next().unwrap().chars().collect::<Vec<char>>();
    people.for_each(|other| first.retain(|&c| other.contains(c)));
    first.len()
}

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    if print_summary {
        output::print_day(6);
    }
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Data
    // -------------------------------------------------------------------------
    // Open file
    let buffer: String = std::fs::read_to_string("data/day06.txt").unwrap();

    // Read to object iterator
    let data: Vec<String> = buffer.split("\n\n").map(|line| line.to_string()).collect();

    // Timing
    let time_setup = start_all.elapsed();

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
    // Timing
    // -------------------------------------------------------------------------
    let time = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let (combined_1, combined_2) = buffer
        .split("\n\n")
        .map(|line| line.to_string())
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
        output::print_setup(time_setup);
        output::print_part(
            1,
            "Count",
            "total yes",
            "✅ Count",
            &format!("{}", count_1),
            time_part_1,
        );
        output::print_part(
            2,
            "Rule",
            "only one of two",
            "✅ Valid",
            &format!("{}", count_2),
            time_part_2,
        );
        output::print_timing(time, time_part_1, time_part_2, time_combined);
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part_1: count_1 as i64,
        part_2: count_2 as i64,
        time: time_combined.as_nanos(),
    };
}

// -----------------------------------------------------------------------------
// Line parsing regex
// -----------------------------------------------------------------------------
//lazy_static! {
//    static ref RE_LINE: Regex = Regex::new(
//        r"(?x)
//            (?P<lower>\d+) # lower requirement
//            -
//            (?P<upper>\d+) # upper requirement
//            \s
//            (?P<required>\w) # required character
//            :\s
//            (?P<password>\w+) # password to check
//            ",
//    )
//    .unwrap();
//}
// -----------------------------------------------------------------------------
