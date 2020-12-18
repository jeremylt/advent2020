//! Day 17:
//! I have experimented with maintaining a hash set of only the active nodes, but this
//! turned into an interesting example where doing lots of the same task is faster than
//! some special case logic to reduce the total FLOPs. In the 4D case, the final solution
//! uses ~1% of the cells, but trying to target active cells and neighbors took 10x longer
//! on my machine. I did use symmetry in the 3rd and 4th dimension to reduce the
//! computation by a factor of nearly 2 and 4, respectively.

use crate::prelude::*;
use arrayvec::ArrayVec;

// Constants
const CAPACITY_3D: usize = 1 << 13;
const CAPACITY_4D: usize = 1 << 16;
const CYCLES: usize = 6;
const NEIGHBOR_LIMIT: usize = 3;

// -----------------------------------------------------------------------------
// Game of Life 3D
// -----------------------------------------------------------------------------
macro_rules! index_3d {
    ($i:expr, $j:expr, $k:expr, $row_length:expr, $column_length:expr) => {
        (($k) * ($column_length) + ($i)) * ($row_length) + ($j)
    };
}

fn count_neighbors_3d(
    i: usize,
    j: usize,
    k: usize,
    row_length: usize,
    column_length: usize,
    indices: &[i32; 26],
    cells: &ArrayVec<[bool; CAPACITY_3D]>,
) -> usize {
    let mut count = 0;
    indices.iter().find_map(|&offset| {
        let active =
            cells[(index_3d!(i, j, k, row_length, column_length) as i32 + offset) as usize];
        if active {
            count += 1;
        }
        if count > NEIGHBOR_LIMIT {
            Some(count)
        } else {
            None
        }
    });
    count
}

fn game_of_life_3d(
    row_length: usize,
    column_length: usize,
    cells: &mut ArrayVec<[bool; CAPACITY_3D]>,
) {
    let row = row_length as i32;
    let column = column_length as i32;
    let neighbors: [i32; 26] = [
        -row * column - row - 1,
        -row * column - row,
        -row * column - row + 1,
        -row * column - 1,
        -row * column,
        -row * column + 1,
        -row * column + row - 1,
        -row * column + row,
        -row * column + row + 1,
        -row - 1,
        -row,
        -row + 1,
        -1,
        1,
        row - 1,
        row,
        row + 1,
        row * column - row - 1,
        row * column - row,
        row * column - row + 1,
        row * column - 1,
        row * column,
        row * column + 1,
        row * column + row - 1,
        row * column + row,
        row * column + row + 1,
    ];

    let mut next_cells = ArrayVec::from([false; CAPACITY_3D]);
    (0..CYCLES).for_each(|cycle| {
        (CYCLES - cycle..row_length - CYCLES + cycle).for_each(|i| {
            (CYCLES - cycle..column_length - CYCLES + cycle).for_each(|j| {
                (1..3 + cycle).for_each(|k| {
                    let count =
                        count_neighbors_3d(i, j, k, row_length, column_length, &neighbors, &cells);
                    let activated = cells[index_3d!(i, j, k, row_length, column_length)];
                    if (activated && (count == 2 || count == 3)) || (!activated && count == 3) {
                        next_cells[index_3d!(i, j, k, row_length, column_length)] = true;
                    } else {
                        next_cells[index_3d!(i, j, k, row_length, column_length)] = false;
                    }
                });
            })
        });
        std::mem::swap(cells, &mut next_cells);
        // Copy periodic slab
        (CYCLES - cycle..row_length - CYCLES + cycle).for_each(|i| {
            (CYCLES - cycle..column_length - CYCLES + cycle).for_each(|j| {
                cells[index_3d!(i, j, 0, row_length, column_length)] =
                    cells[index_3d!(i, j, 2, row_length, column_length)];
            })
        });
    });
}

// -----------------------------------------------------------------------------
// Game of Life 4D
// -----------------------------------------------------------------------------
macro_rules! index_4d {
    ($i:expr, $j:expr, $k:expr, $l:expr, $row_length:expr, $column_length:expr) => {
        ((($l) * (CYCLES + 3) + ($k)) * ($column_length) + ($i)) * ($row_length) + ($j)
    };
}

fn count_neighbors_4d(
    i: usize,
    j: usize,
    k: usize,
    l: usize,
    row_length: usize,
    column_length: usize,
    indices: &[i32; 80],
    cells: &ArrayVec<[bool; CAPACITY_4D]>,
) -> usize {
    let mut count = 0;
    indices.iter().find_map(|&offset| {
        let active =
            cells[(index_4d!(i, j, k, l, row_length, column_length) as i32 + offset) as usize];
        if active {
            count += 1;
        }
        if count > NEIGHBOR_LIMIT {
            Some(count)
        } else {
            None
        }
    });
    count
}

fn game_of_life_4d(
    row_length: usize,
    column_length: usize,
    cells: &mut ArrayVec<[bool; CAPACITY_4D]>,
) {
    let row = row_length as i32;
    let column = column_length as i32;
    let slab = row * column * (CYCLES + 3) as i32;
    let neighbors: [i32; 80] = [
        -slab - row * column - row - 1,
        -slab - row * column - row,
        -slab - row * column - row + 1,
        -slab - row * column - 1,
        -slab - row * column,
        -slab - row * column + 1,
        -slab - row * column + row - 1,
        -slab - row * column + row,
        -slab - row * column + row + 1,
        -slab - row - 1,
        -slab - row,
        -slab - row + 1,
        -slab - 1,
        -slab,
        -slab + 1,
        -slab + row - 1,
        -slab + row,
        -slab + row + 1,
        -slab + row * column - row - 1,
        -slab + row * column - row,
        -slab + row * column - row + 1,
        -slab + row * column - 1,
        -slab + row * column,
        -slab + row * column + 1,
        -slab + row * column + row - 1,
        -slab + row * column + row,
        -slab + row * column + row + 1,
        -row * column - row - 1,
        -row * column - row,
        -row * column - row + 1,
        -row * column - 1,
        -row * column,
        -row * column + 1,
        -row * column + row - 1,
        -row * column + row,
        -row * column + row + 1,
        -row - 1,
        -row,
        -row + 1,
        -1,
        1,
        row - 1,
        row,
        row + 1,
        row * column - row - 1,
        row * column - row,
        row * column - row + 1,
        row * column - 1,
        row * column,
        row * column + 1,
        row * column + row - 1,
        row * column + row,
        row * column + row + 1,
        slab - row * column - row - 1,
        slab - row * column - row,
        slab - row * column - row + 1,
        slab - row * column - 1,
        slab - row * column,
        slab - row * column + 1,
        slab - row * column + row - 1,
        slab - row * column + row,
        slab - row * column + row + 1,
        slab - row - 1,
        slab - row,
        slab - row + 1,
        slab - 1,
        slab,
        slab + 1,
        slab + row - 1,
        slab + row,
        slab + row + 1,
        slab + row * column - row - 1,
        slab + row * column - row,
        slab + row * column - row + 1,
        slab + row * column - 1,
        slab + row * column,
        slab + row * column + 1,
        slab + row * column + row - 1,
        slab + row * column + row,
        slab + row * column + row + 1,
    ];

    let mut next_cells = ArrayVec::from([false; CAPACITY_4D]);
    (0..CYCLES).for_each(|cycle| {
        (CYCLES - cycle..row_length - CYCLES + cycle).for_each(|i| {
            (CYCLES - cycle..column_length - CYCLES + cycle).for_each(|j| {
                (1..3 + cycle).for_each(|k| {
                    (1..3 + cycle).for_each(|l| {
                        let count = count_neighbors_4d(
                            i,
                            j,
                            k,
                            l,
                            row_length,
                            column_length,
                            &neighbors,
                            &cells,
                        );
                        let activated = cells[index_4d!(i, j, k, l, row_length, column_length)];
                        if (activated && (count == 2 || count == 3)) || (!activated && count == 3) {
                            next_cells[index_4d!(i, j, k, l, row_length, column_length)] = true;
                        } else {
                            next_cells[index_4d!(i, j, k, l, row_length, column_length)] = false;
                        }
                    })
                })
            })
        });
        std::mem::swap(cells, &mut next_cells);
        // Copy periodic slabs
        (CYCLES - cycle..row_length - CYCLES + cycle).for_each(|i| {
            (CYCLES - cycle..column_length - CYCLES + cycle).for_each(|j| {
                cells[index_4d!(i, j, 0, 0, row_length, column_length)] =
                    cells[index_4d!(i, j, 2, 2, row_length, column_length)];
                (1..3 + cycle).for_each(|k| {
                    cells[index_4d!(i, j, k, 0, row_length, column_length)] =
                        cells[index_4d!(i, j, k, 2, row_length, column_length)];
                });
                (1..3 + cycle).for_each(|l| {
                    cells[index_4d!(i, j, 0, l, row_length, column_length)] =
                        cells[index_4d!(i, j, 2, l, row_length, column_length)];
                });
            })
        });
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
    let buffer: String = std::fs::read_to_string("data/day17.txt").unwrap();

    // Read to vector
    let row_length = buffer.lines().nth(0).unwrap().chars().count() + 2 * (CYCLES + 1);
    let column_length = buffer.lines().count() + 2 * (CYCLES + 1);
    let mut cells: ArrayVec<[bool; CAPACITY_3D]> = ArrayVec::from([false; CAPACITY_3D]);
    buffer.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                cells[index_3d!(i + CYCLES + 1, j + CYCLES + 1, 1, row_length, column_length)] =
                    true
            }
        })
    });
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find 3D initialization
    let start_part_1 = Instant::now();
    game_of_life_3d(row_length, column_length, &mut cells);
    let count_1 = cells
        .iter()
        .skip(row_length * column_length)
        .take(row_length * column_length)
        .filter(|&cell| *cell)
        .count()
        + 2 * cells
            .iter()
            .skip(2 * row_length * column_length)
            .filter(|&cell| *cell)
            .count();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find 4D initialization
    let start_part_2 = Instant::now();
    let mut cells: ArrayVec<[bool; CAPACITY_4D]> = ArrayVec::from([false; CAPACITY_4D]);
    buffer.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                cells[index_4d!(
                    i + CYCLES + 1,
                    j + CYCLES + 1,
                    1,
                    1,
                    row_length,
                    column_length
                )] = true
            }
        })
    });
    game_of_life_4d(row_length, column_length, &mut cells);
    let mut count_2 = 0;
    (1..row_length - 1).for_each(|i| {
        (1..column_length - 1).for_each(|j| {
            // Not repeated
            if cells[index_4d!(i, j, 1, 1, row_length, column_length)] {
                count_2 += 1;
            }
            // Repeated twice
            (2..CYCLES + 2).for_each(|k| {
                if cells[index_4d!(i, j, k, 1, row_length, column_length)] {
                    count_2 += 2;
                }
            });
            (2..CYCLES + 2).for_each(|l| {
                // Repeated twice
                if cells[index_4d!(i, j, 1, l, row_length, column_length)] {
                    count_2 += 2;
                }
                // Repeated four times
                (2..CYCLES + 2).for_each(|k| {
                    if cells[index_4d!(i, j, k, l, row_length, column_length)] {
                        count_2 += 4;
                    }
                });
            });
        })
    });
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
    output::print_day(17, "Conway Cubes");
    output::print_part(1, "🛰 Activated", &format!("{}", results.part_1));
    output::print_part(2, "🛰 Activated", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
