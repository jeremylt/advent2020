//! Day 12:
//! Another straightforward problem today. As usual, for these easier problems parsing
//! the data is expensive. Since the trig is only by 90 degrees, it is faster to just
//! directly handle the rotation matrix effects in part 2.

use crate::prelude::*;

// -----------------------------------------------------------------------------
// Read line to array
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Instruction {
    north: i16,
    east: i16,
    bearing: i16,
    forward: i16,
}

impl std::str::FromStr for Instruction {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse::<i16>()?;
        let (mut north, mut east, mut bearing, mut forward) = (0, 0, 0, 0);
        match s.as_bytes()[0] {
            b'N' => north = value,
            b'S' => north = -value,
            b'E' => east = value,
            b'W' => east = -value,
            b'L' => bearing = value,
            b'R' => bearing = 360 - value,
            b'F' => forward = value,
            _ => panic!("unknown instruction"),
        }
        Ok(Self {
            north,
            east,
            bearing,
            forward,
        })
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Position {
    north: i16,
    east: i16,
    bearing: i16,
}

#[inline(always)]
fn part_1(current: &Position, instruction: &Instruction) -> Position {
    let mut result = Position {
        north: current.north,
        east: current.east,
        bearing: current.bearing,
    };
    if instruction.bearing != 0 {
        // Update bearing
        result.bearing = (current.bearing + instruction.bearing) % 360;
    } else if instruction.forward > 0 {
        // Move 'forward'
        match current.bearing {
            0 => result.east += instruction.forward,
            90 => result.north += instruction.forward,
            180 => result.east -= instruction.forward,
            270 => result.north -= instruction.forward,
            _ => panic!("unknown direction"),
        }
    } else {
        // Move in direction
        result.north += instruction.north;
        result.east += instruction.east;
    }
    result
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct PositionWaypoint {
    north: i16,
    east: i16,
    waypoint_north: i16,
    waypoint_east: i16,
}
#[inline(always)]
fn part_2(current: &PositionWaypoint, instruction: &Instruction) -> PositionWaypoint {
    let mut result = PositionWaypoint {
        north: current.north,
        east: current.east,
        waypoint_north: current.waypoint_north,
        waypoint_east: current.waypoint_east,
    };
    if instruction.bearing != 0 {
        // Rotate waypoint orientation
        match instruction.bearing {
            90 => {
                result.waypoint_north = current.waypoint_east;
                result.waypoint_east = -current.waypoint_north;
            }
            180 => {
                result.waypoint_north *= -1;
                result.waypoint_east *= -1;
            }
            270 => {
                result.waypoint_north = -current.waypoint_east;
                result.waypoint_east = current.waypoint_north;
            }
            _ => panic!("unknown direction"),
        }
    } else if instruction.forward > 0 {
        // Move "forward" towards waypoint
        result.north += instruction.forward * current.waypoint_north;
        result.east += instruction.forward * current.waypoint_east;
    } else {
        // Move waypoint
        result.waypoint_north += instruction.north;
        result.waypoint_east += instruction.east;
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
    let values: Vec<Instruction> = buffer
        .lines()
        .map(|line| line.parse::<Instruction>().expect("failed to parse line"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Move in directions given
    let start_part_1 = Instant::now();
    let position_1 = values.iter().fold(
        Position {
            north: 0,
            east: 0,
            bearing: 0,
        },
        |acc, instruction| part_1(&acc, instruction),
    );
    let distance_1 = position_1.north.abs() + position_1.east.abs();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Move towards waypoint
    let start_part_2 = Instant::now();
    let position_2 = values.iter().fold(
        PositionWaypoint {
            north: 0,
            east: 0,
            waypoint_north: 1,
            waypoint_east: 10,
        },
        |acc, instruction| part_2(&acc, instruction),
    );
    let distance_2 = position_2.north.abs() + position_2.east.abs();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let (result_1, result_2) = buffer
        .lines()
        .map(|line| line.parse::<Instruction>().expect("failed to parse line"))
        .fold(
            (
                Position {
                    north: 0,
                    east: 0,
                    bearing: 0,
                },
                PositionWaypoint {
                    north: 0,
                    east: 0,
                    waypoint_north: 1,
                    waypoint_east: 10,
                },
            ),
            |acc, instruction| (part_1(&acc.0, &instruction), part_2(&acc.1, &instruction)),
        );
    let (combined_1, combined_2) = (
        result_1.north.abs() + result_1.east.abs(),
        result_2.north.abs() + result_2.east.abs(),
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
