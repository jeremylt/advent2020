use crate::prelude::*;
use itertools::Itertools;

// -----------------------------------------------------------------------------
// Parse FBLR encoded binary
// -----------------------------------------------------------------------------
fn parse_fblr_binary(s: &str) -> usize {
    s.chars().fold(0, |id, c| {
        id * 2 + if ['B', 'R'].contains(&c) { 1 } else { 0 }
    })
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(triple: (&bool, &bool, &bool)) -> bool {
    *triple.0 && !*triple.1 && *triple.2
}

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    if print_summary {
        output::print_day(5);
    }
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Data
    // -------------------------------------------------------------------------
    // Open file
    let buffer = std::fs::read_to_string("data/day05.txt").unwrap();

    // Read to object iterator
    let data: Vec<usize> = buffer
        .lines()
        .map(|line| parse_fblr_binary(&line))
        .collect();

    // Timing
    let time_setup = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passports
    let start_part_1 = Instant::now();
    let max_1 = *data.iter().max().unwrap();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passports
    let start_part_2 = Instant::now();
    let mut mask = [false; 2 << 9];
    data.iter().for_each(|&s| mask[s] = true);
    let seat_2 = mask
        .iter()
        .tuple_windows::<(_, _, _)>()
        .enumerate()
        .find_map(|(i, t)| if part_2(t) { Some(i as i32) } else { None })
        .unwrap()
        + 1;
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
            "Rule",
            "find largest seat",
            "💺 Largest",
            &format!("{}", max_1),
            time_part_1,
        );
        output::print_part(
            2,
            "Rule",
            "find emply seat",
            "💺 Avaliable",
            &format!("{}", seat_2),
            time_part_2,
        );
        output::print_timing(time, time_part_1, time_part_2);
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part_1: max_1 as i64,
        part_2: seat_2 as i64,
        time: time.as_nanos(),
    };
}

// -----------------------------------------------------------------------------
