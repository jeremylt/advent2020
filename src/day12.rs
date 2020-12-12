//! Day 12:
//! Another straightforward problem today. As usual, for these easier problems parsing
//! the data is expensive. Since the trig is only by 90 degrees, it is faster to just
//! directly handle the rotation matrix effects in part 2.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Read line to array
// -----------------------------------------------------------------------------
#[inline(always)]
fn to_array(s: &str) -> Result<[i16; 4], std::num::ParseIntError> {
    let value = s[1..].parse::<i16>()?;
    match s.as_bytes()[0] {
        b'N' => Ok([value, 0, 0, 0]),
        b'S' => Ok([-value, 0, 0, 0]),
        b'E' => Ok([0, value, 0, 0]),
        b'W' => Ok([0, -value, 0, 0]),
        b'L' => Ok([0, 0, value, 0]),
        b'R' => Ok([0, 0, 360 - value, 0]),
        b'F' => Ok([0, 0, 0, value]),
        _ => panic!("unknown instruction"),
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
#[inline(always)]
fn part_1(current: &[i16; 3], instruction: &[i16; 4]) -> [i16; 3] {
    let result: [i16; 3];
    let [north, east, degree] = *current;
    if instruction[2] != 0 {
        // Update degree
        result = [north, east, (degree + instruction[2]) % 360];
    } else if instruction[3] > 0 {
        // Move 'forward'
        let forward = instruction[3];
        match current[2] {
            0 => result = [north, east + forward, degree],
            90 => result = [north + forward, east, degree],
            180 => result = [north, east - forward, degree],
            270 => result = [north - forward, east, degree],
            _ => panic!("unknown direction"),
        }
    } else {
        // Move in direction
        result = [north + instruction[0], east + instruction[1], degree];
    }
    result
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
#[inline(always)]
fn part_2(current: &[i16; 4], instruction: &[i16; 4]) -> [i16; 4] {
    let result: [i16; 4];
    let [north, east, waypoint_north, waypoint_east] = *current;
    if instruction[2] != 0 {
        // Rotate waypoint orientation
        let degree = instruction[2];
        match degree {
            90 => result = [north, east, waypoint_east, -waypoint_north],
            180 => result = [north, east, -waypoint_north, -waypoint_east],
            270 => result = [north, east, -waypoint_east, waypoint_north],
            _ => panic!("unknown direction"),
        }
    } else if instruction[3] > 0 {
        // Move "forward" towards waypoint
        let forward = instruction[3];
        result = [
            north + forward * waypoint_north,
            east + forward * waypoint_east,
            waypoint_north,
            waypoint_east,
        ];
    } else {
        // Move waypoint
        result = [
            north,
            east,
            waypoint_north + instruction[0],
            waypoint_east + instruction[1],
        ];
    }
    result
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
    let buffer: String = std::fs::read_to_string("data/day12.txt").unwrap();

    // Read to vector
    let values: Vec<[i16; 4]> = buffer
        .lines()
        .map(|line| to_array(line).expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Move in directions given
    let start_part_1 = Instant::now();
    let [north_1, east_1, _] = values
        .iter()
        .fold([0, 0, 0], |acc, instruction| part_1(&acc, instruction));
    let distance_1 = north_1.abs() + east_1.abs();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Move towards waypoint
    let start_part_2 = Instant::now();
    let [north_2, east_2, _, _] = values
        .iter()
        .fold([0, 0, 1, 10], |acc, instruction| part_2(&acc, instruction));
    let distance_2 = north_2.abs() + east_2.abs();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let (result_1, result_2) = buffer
        .lines()
        .map(|line| to_array(line).expect("failed to parse line"))
        .fold(([0; 3], [0, 0, 1, 10]), |acc, instruction| {
            (part_1(&acc.0, &instruction), part_2(&acc.1, &instruction))
        });
    let (combined_1, combined_2) = (
        result_1[0].abs() + result_1[1].abs(),
        result_2[0].abs() + result_2[1].abs(),
    );
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, distance_1);
    assert_eq!(combined_2, distance_2);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        distance_1 as i64,
        distance_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(12, "Rain Risk");
    output::print_part(1, "🧭 Distance", &format!("{}", results.part_1));
    output::print_part(2, "🧭 Distance", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
