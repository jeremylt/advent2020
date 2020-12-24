//! Day 24:
//! Game of Life on a hexagonal grid. I largely recycled older code, with some new
//! code for parsing a line into a location onto a 2D hexagonal coordinate system.

use crate::prelude::*;

// Constants
const GENERATIONS: usize = 100;
const GRID_SIZE: usize = 2 * GENERATIONS + 34;
const OFFSET: usize = GRID_SIZE / 2;

// -----------------------------------------------------------------------------
// Hexagonal coordinates
// -----------------------------------------------------------------------------
#[derive(Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    east: i16,
    north_east: i16,
}

// 2D Hexagonal coordinate system
//
//        / \   / \
//       /   \ /   \     __
//      |-1,1 | 0,1 |     /| Northeast
//      |     |     |    /
//     / \   / \   / \
//    /   \ /   \ /   \
//   |-1,0 | 0,0 | 1,0 | --> East
//   |     |     |     |
//    \   / \   / \   /
//     \ /   \ /   \ /
//      |0,-1 |1,-1 |
//      |     |     |
//       \   / \   /
//        \ /   \ /
//
impl std::str::FromStr for Coordinate {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut east, mut north_east) = (0, 0);
        let mut i = 0;
        while i < s.len() {
            if s.as_bytes()[i] == b'e' {
                // East
                east += 1;
                i += 1;
            } else if s.as_bytes()[i] == b'w' {
                // Anti-east (west)
                east -= 1;
                i += 1;
            } else if s.as_bytes()[i] == b'n' {
                // Northeast
                north_east += 1;
                i += 2;
                if s.as_bytes()[i - 1] == b'w' {
                    // Northeast + anti-east (northwest)
                    east -= 1;
                }
            } else {
                // Anti-northeast (southwest)
                north_east -= 1;
                i += 2;
                if s.as_bytes()[i - 1] == b'e' {
                    // Anti-northeast + east (southeast)
                    east += 1;
                }
            }
        }
        Ok(Coordinate { east, north_east })
    }
}

impl Coordinate {
    fn to_index(&self) -> usize {
        ((OFFSET as i16 + self.east) as usize)
            + ((OFFSET as i16 + self.north_east) as usize) * GRID_SIZE
    }
}

// -----------------------------------------------------------------------------
// Hexagonal Game of Life
// -----------------------------------------------------------------------------
macro_rules! index_2d {
    ($i:expr, $j:expr, $row_length:expr) => {
        ($i) * ($row_length) + ($j)
    };
}

fn game_of_life(tiles: &mut [bool; GRID_SIZE * GRID_SIZE], generations: usize) {
    let offsets = [
        -1,
        1,
        -(GRID_SIZE as i16),
        GRID_SIZE as i16,
        -1 + GRID_SIZE as i16,
        1 - (GRID_SIZE as i16),
    ];
    let mut next_tiles = [false; GRID_SIZE * GRID_SIZE];

    let min_window = GENERATIONS;
    let max_window = GRID_SIZE - GENERATIONS;
    (0..generations).for_each(|generation| {
        (min_window - generation..max_window + generation).for_each(|i| {
            (min_window - generation..max_window + generation).for_each(|j| {
                let index = index_2d!(i, j, GRID_SIZE);
                let count = offsets.iter().fold(tiles[index] as u8, |acc, &offset| {
                    (acc << 1) | tiles[(index as i32 + offset as i32) as usize] as u8
                });
                next_tiles[index] = NEIGHBORS[count as usize];
            });
        });
        std::mem::swap(tiles, &mut next_tiles);
    });
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
    let buffer: String = std::fs::read_to_string("data/day24.txt").unwrap();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Count initial tiles
    let start_part_1 = Instant::now();
    let mut tiles = [false; GRID_SIZE * GRID_SIZE];
    buffer.lines().for_each(|line| {
        let coordinate = line
            .parse::<Coordinate>()
            .expect("failed to parse directions");
        tiles[coordinate.to_index()] ^= true;
    });
    let count_1 = tiles.iter().filter(|&tile| *tile).count();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Run Game of Life
    let start_part_2 = Instant::now();
    game_of_life(&mut tiles, GENERATIONS);
    let count_2 = tiles.iter().filter(|&tile| *tile).count();
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
    output::print_day(24, "Lobby Layout");
    output::print_part(1, "🏨 Count", &format!("{}", results.part_1));
    output::print_part(2, "🏨 Count", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
// Boolean array for neighbor states
// -----------------------------------------------------------------------------
const NEIGHBORS: [bool; 1 << 7] = [
    false, false, false, true, false, true, true, false, false, true, true, false, true, false,
    false, false, false, true, true, false, true, false, false, false, true, false, false, false,
    false, false, false, false, false, true, true, false, true, false, false, false, true, false,
    false, false, false, false, false, false, true, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, true, true, true,
    true, true, false, true, true, true, false, true, false, false, false, true, true, true, false,
    true, false, false, false, true, false, false, false, false, false, false, false, true, true,
    true, false, true, false, false, false, true, false, false, false, false, false, false, false,
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false,
];

// -----------------------------------------------------------------------------
