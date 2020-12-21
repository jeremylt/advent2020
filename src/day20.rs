//! Day 20:
//! Well then. That was hard, but the overall implementation was somewhat fast.

use crate::prelude::*;

// Constants
const TILE_SIZE: usize = 10;
const MAX_TILE_SIDE: usize = 1024; // 2 ^ 10
const SEA_MONSTER_SIZE: usize = 15;

// -----------------------------------------------------------------------------
// Tile with orientation and rotation
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Tile {
    id: u16,
    image: Vec<bool>,
    side: Side,
    orientation: Orientation,
    edges: [Edge; 4],
    shared_edges: u8,
}

// Tiles are structured as
//
//   <---Edge 1---
//  |            /|\
//  E  .#.#.#.#.  E
//  d  #.#.#.#.#  d
//  g  .#.#.#.#.  g
//  e  #.#.#.#.#  e
//     .#.#.#.#.
//  2  #.#.#.#.#  0
// \|/            |
//   ---Edge 3--->
//
// `Orientation::Up` means the edges are numbered radianswise
// `Orientation::Down` means the edges are numbered antiradianswise
//
// `Rotation::{Right, Top, Left, Bottom}` indicates the location of Edge 0
//
// `edges` are encoded as u16 values encoded from the boolean from
//     traversing the edge radianwise and antiradianswise
//
// `image` is a boolean array, with `true` for every '#'
//
// 'shared_edges' counts the number of edges that match other tiles
//    Note, this value will be a multiple of 2, for radians and antiradians

impl std::str::FromStr for Tile {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.splitn(2, "\n");
        let id = data.next().unwrap()[5..9].parse()?;
        let image: Vec<bool> = data
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| if c == '\n' { None } else { Some(c == '#') })
            .collect();
        let side = Side::Up;
        let orientation = Orientation::Right;
        let (antiradians_0, radians_0) = image
            .iter()
            .skip(TILE_SIZE - 1)
            .step_by(TILE_SIZE)
            .take(TILE_SIZE)
            .enumerate()
            .fold((0, 0), |acc, (i, &c)| {
                let bit = if c { 1 } else { 0 };
                (acc.0 + (bit << (TILE_SIZE - 1 - i)), acc.1 + (bit << i))
            });
        let (antiradians_1, radians_1) =
            image
                .iter()
                .take(TILE_SIZE)
                .enumerate()
                .fold((0, 0), |acc, (i, &c)| {
                    let bit = if c { 1 } else { 0 };
                    (acc.0 + (bit << (TILE_SIZE - 1 - i)), acc.1 + (bit << i))
                });
        let (radians_2, antiradians_2) = image
            .iter()
            .step_by(TILE_SIZE)
            .take(TILE_SIZE)
            .enumerate()
            .fold((0, 0), |acc, (i, &c)| {
                let bit = if c { 1 } else { 0 };
                (acc.0 + (bit << (TILE_SIZE - 1 - i)), acc.1 + (bit << i))
            });
        let (radians_3, antiradians_3) = image
            .iter()
            .skip(TILE_SIZE * (TILE_SIZE - 1))
            .take(TILE_SIZE)
            .enumerate()
            .fold((0, 0), |acc, (i, &c)| {
                let bit = if c { 1 } else { 0 };
                (acc.0 + (bit << (TILE_SIZE - 1 - i)), acc.1 + (bit << i))
            });
        let edges = [
            Edge {
                radians: radians_0,
                antiradians: antiradians_0,
            },
            Edge {
                radians: radians_1,
                antiradians: antiradians_1,
            },
            Edge {
                radians: radians_2,
                antiradians: antiradians_2,
            },
            Edge {
                radians: radians_3,
                antiradians: antiradians_3,
            },
        ];
        let shared_edges = 0;
        Ok(Self {
            id,
            image,
            side,
            orientation,
            edges,
            shared_edges,
        })
    }
}

impl Tile {
    // Get current right edge
    fn get_right(&self) -> u16 {
        let index = match &self.orientation {
            Orientation::Right => 0,
            Orientation::Top => 3,
            Orientation::Left => 2,
            Orientation::Bottom => 1,
        };
        match &self.side {
            Side::Up => self.edges[(index as usize)].radians,
            Side::Down => self.edges[(4 - index as usize) % 4].antiradians,
        }
    }

    // Get current bottom edge
    fn get_bottom(&self) -> u16 {
        let index = match &self.orientation {
            Orientation::Right => 3,
            Orientation::Top => 2,
            Orientation::Left => 1,
            Orientation::Bottom => 0,
        };
        match &self.side {
            Side::Up => self.edges[(index as usize) % 4].radians,
            Side::Down => self.edges[(4 - index as usize) % 4].antiradians,
        }
    }

    // Flip tile
    fn flip(&mut self) {
        self.side = if self.side == Side::Up {
            Side::Down
        } else {
            Side::Up
        };
    }

    // Set left edge
    fn set_left(&mut self, target_edge: u16) {
        let mut flip = false;
        let index = self.edges.iter().enumerate().find_map(|(i, edge)| {
            if edge.antiradians == target_edge {
                Some(i)
            } else if edge.radians == target_edge {
                flip = true;
                Some(i)
            } else {
                None
            }
        });
        if flip {
            self.flip();
        }
        match index {
            Some(value) => match value {
                0 => self.orientation = Orientation::Left,
                2 => self.orientation = Orientation::Right,
                1 => {
                    self.orientation = if self.side == Side::Up {
                        Orientation::Top
                    } else {
                        Orientation::Bottom
                    }
                }
                3 => {
                    self.orientation = if self.side == Side::Up {
                        Orientation::Bottom
                    } else {
                        Orientation::Top
                    }
                }
                _ => unreachable!(),
            },
            None => panic!("invalid index"),
        };
    }

    // Set top edge
    fn set_top(&mut self, target_edge: u16) {
        let mut flip = false;
        let index = self.edges.iter().enumerate().find_map(|(i, edge)| {
            if edge.antiradians == target_edge {
                Some(i)
            } else if edge.radians == target_edge {
                flip = true;
                Some(i)
            } else {
                None
            }
        });
        if flip {
            self.flip();
        }
        match index {
            Some(value) => match value {
                0 => self.orientation = Orientation::Top,
                2 => self.orientation = Orientation::Bottom,
                1 => {
                    self.orientation = if self.side == Side::Up {
                        Orientation::Right
                    } else {
                        Orientation::Left
                    }
                }
                3 => {
                    self.orientation = if self.side == Side::Up {
                        Orientation::Left
                    } else {
                        Orientation::Right
                    }
                }
                _ => unreachable!(),
            },
            None => panic!("invalid index"),
        };
    }

    // Set top corner
    fn set_top_left(&mut self, corner: &[u16]) {
        let index_1 = self.edges.iter().enumerate().find_map(|(i, edge)| {
            if edge.radians == corner[0] {
                Some(i)
            } else {
                None
            }
        });
        let index_2 = self.edges.iter().enumerate().find_map(|(i, edge)| {
            if edge.radians == corner[1] {
                Some(i)
            } else {
                None
            }
        });
        let top = std::cmp::min(index_1.unwrap(), index_2.unwrap());
        self.set_top(self.edges[top].antiradians);
    }
}

#[derive(Debug, PartialEq)]
enum Side {
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
enum Orientation {
    Right,
    Top,
    Left,
    Bottom,
}

#[derive(Debug)]
struct Edge {
    radians: u16,
    antiradians: u16,
}

// -----------------------------------------------------------------------------
// Find sea monster
// -----------------------------------------------------------------------------
#[inline]
fn is_monster(
    head: usize,
    row_size: usize,
    monster_indices: &[(i32, i32); 14],
    image: &Vec<bool>,
) -> bool {
    image[head]
        && monster_indices
            .iter()
            .all(|(i, j)| image[head + (i * row_size as i32 + j) as usize])
}

// -----------------------------------------------------------------------------
// Macros
// -----------------------------------------------------------------------------
macro_rules! index_2d {
    ($i:expr, $j:expr, $side_length:expr) => {
        ($i) * ($side_length) + ($j)
    };
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
    let buffer: String = std::fs::read_to_string("data/day20.txt").unwrap();

    let mut pairs = [[MAX_TILE_SIDE; 2]; MAX_TILE_SIDE];
    let mut tiles: Vec<Tile> = buffer
        .split("\n\n")
        .enumerate()
        .map(|(i, tile_string)| {
            let tile = tile_string.parse::<Tile>().expect("failed to parse tile");
            tile.edges.iter().for_each(|edge| {
                let index = if pairs[edge.radians as usize][0] == MAX_TILE_SIDE {
                    0
                } else {
                    1
                };
                pairs[edge.radians as usize][index] = i;
                if edge.radians != edge.antiradians {
                    let index = if pairs[edge.antiradians as usize][0] == MAX_TILE_SIDE {
                        0
                    } else {
                        1
                    };
                    pairs[edge.antiradians as usize][index] = i;
                }
            });
            tile
        })
        .collect();
    let number_tiles = tiles.len();
    let side_length = (number_tiles as f64).sqrt() as usize;
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find product of corners
    let start_part_1 = Instant::now();
    pairs.iter().for_each(|pair| {
        if pair[0] != MAX_TILE_SIDE && pair[1] != MAX_TILE_SIDE {
            tiles[pair[0]].shared_edges += 1;
            tiles[pair[1]].shared_edges += 1;
        }
    });
    let mut corner = number_tiles;
    let product_1: usize = tiles
        .iter()
        .enumerate()
        .filter_map(|(i, tile)| {
            if tile.shared_edges == 4 {
                corner = i as usize;
                Some(tile.id as usize)
            } else {
                None
            }
        })
        .product();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Assemble image and count sea monsters
    let start_part_2 = Instant::now();
    // Setup corner
    let corner_edges: Vec<u16> = tiles[corner]
        .edges
        .iter()
        .filter_map(|edge| {
            if pairs[edge.radians as usize][1] == MAX_TILE_SIDE {
                Some(edge.radians)
            } else {
                None
            }
        })
        .collect();
    tiles[corner].set_top_left(&corner_edges);

    // Fill grid and image
    let mut grid = vec![0; number_tiles];
    grid[0] = corner;
    let row_size = side_length * (TILE_SIZE - 2);
    let mut image = vec![false; row_size * row_size];

    (0..side_length).for_each(|i| {
        (0..side_length).for_each(|j| {
            // Find next tile
            if j == 0 {
                // Match on top
                if i != 0 {
                    let target_index = grid[index_2d!(i - 1, j, side_length)];
                    let target_edge = tiles[target_index].get_bottom();
                    let next_tile = if pairs[target_edge as usize][0] != target_index {
                        pairs[target_edge as usize][0]
                    } else {
                        pairs[target_edge as usize][1]
                    };
                    tiles[next_tile].set_top(target_edge);
                    grid[index_2d!(i, j, side_length)] = next_tile;
                }
            } else {
                // Match on left
                let target_index = grid[index_2d!(i, j - 1, side_length)];
                let target_edge = tiles[target_index].get_right();
                let next_tile = if pairs[target_edge as usize][0] != target_index {
                    pairs[target_edge as usize][0]
                } else {
                    pairs[target_edge as usize][1]
                };
                tiles[next_tile].set_left(target_edge);
                grid[index_2d!(i, j, side_length)] = next_tile;
            }
            // Add to tile image
            let tile = &tiles[grid[index_2d!(i, j, side_length)]];
            let (step_i, step_j, start): (i32, i32, i32) = match (&tile.side, &tile.orientation) {
                (Side::Up, Orientation::Right) => TILE_INDICES[0],
                (Side::Up, Orientation::Top) => TILE_INDICES[1],
                (Side::Up, Orientation::Left) => TILE_INDICES[2],
                (Side::Up, Orientation::Bottom) => TILE_INDICES[3],
                (Side::Down, Orientation::Right) => TILE_INDICES[4],
                (Side::Down, Orientation::Top) => TILE_INDICES[5],
                (Side::Down, Orientation::Left) => TILE_INDICES[6],
                (Side::Down, Orientation::Bottom) => TILE_INDICES[7],
            };
            let offset = i * row_size * (TILE_SIZE - 2) + j * (TILE_SIZE - 2);
            (0..TILE_SIZE - 2).for_each(|ii| {
                (0..TILE_SIZE - 2).for_each(|jj| {
                    image[offset + ii * row_size + jj] =
                        tile.image[(ii as i32 * step_i + jj as i32 * step_j + start) as usize];
                })
            });
        })
    });

    // Count sea monsters
    let total_features = image.iter().filter(|&p| *p).count();
    let sea_monster_count = SEA_MONSTER_INDICES
        .iter()
        .enumerate()
        .find_map(|(index, &indices)| {
            let mut count = 0;
            (SEA_MONSTER_OFFSETS[index].0..row_size - SEA_MONSTER_OFFSETS[index].1).for_each(|i| {
                (SEA_MONSTER_OFFSETS[index].2..row_size - SEA_MONSTER_OFFSETS[index].3).for_each(
                    |j| {
                        if is_monster(index_2d!(i, j, row_size), row_size, &indices, &image) {
                            count += 1;
                        }
                    },
                )
            });
            if count > 0 {
                Some(count)
            } else {
                None
            }
        })
        .unwrap();

    let count_2 = total_features - sea_monster_count * SEA_MONSTER_SIZE;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        product_1 as i64,
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
    output::print_day(20, "Jurassic Jigsaw");
    output::print_part(1, "🎞 Product", &format!("{}", results.part_1));
    output::print_part(2, "🎞 Count", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
// Sea monster shapes
// -----------------------------------------------------------------------------
const SEA_MONSTER_OFFSETS: [(usize, usize, usize, usize); 8] = [
    (0, 2, 18, 1),
    (0, 2, 1, 18),
    (2, 0, 18, 1),
    (2, 0, 1, 18),
    (18, 1, 0, 2),
    (1, 18, 0, 2),
    (18, 1, 2, 0),
    (1, 18, 2, 0),
];

const SEA_MONSTER_INDICES: [[(i32, i32); 14]; 8] = [
    [
        (1, -18),
        (1, -13),
        (1, -12),
        (1, -7),
        (1, -6),
        (1, -1),
        (1, 0),
        (1, 1),
        (2, -17),
        (2, -14),
        (2, -11),
        (2, -8),
        (2, -5),
        (2, -2),
    ],
    [
        (1, 18),
        (1, 13),
        (1, 12),
        (1, 7),
        (1, 6),
        (1, 1),
        (1, 0),
        (1, -1),
        (2, 17),
        (2, 14),
        (2, 11),
        (2, 8),
        (2, 5),
        (2, 2),
    ],
    [
        (-1, -18),
        (-1, -13),
        (-1, -12),
        (-1, -7),
        (-1, -6),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (-2, -17),
        (-2, -14),
        (-2, -11),
        (-2, -8),
        (-2, -5),
        (-2, -2),
    ],
    [
        (-1, 18),
        (-1, 13),
        (-1, 12),
        (-1, 7),
        (-1, 6),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (-2, 17),
        (-2, 14),
        (-2, 11),
        (-2, 8),
        (-2, 5),
        (-2, 2),
    ],
    [
        (-18, 1),
        (-13, 1),
        (-12, 1),
        (-7, 1),
        (-6, 1),
        (-1, 1),
        (0, 1),
        (1, 1),
        (-17, 2),
        (-14, 2),
        (-11, 2),
        (-8, 2),
        (-5, 2),
        (-2, 2),
    ],
    [
        (18, 1),
        (13, 1),
        (12, 1),
        (7, 1),
        (6, 1),
        (1, 1),
        (0, 1),
        (-1, 1),
        (17, 2),
        (14, 2),
        (11, 2),
        (8, 2),
        (5, 2),
        (2, 2),
    ],
    [
        (-18, -1),
        (-13, -1),
        (-12, -1),
        (-7, -1),
        (-6, -1),
        (-1, -1),
        (0, -1),
        (1, -1),
        (-17, -2),
        (-14, -2),
        (-11, -2),
        (-8, -2),
        (-5, -2),
        (-2, -2),
    ],
    [
        (18, -1),
        (13, -1),
        (12, -1),
        (7, -1),
        (6, -1),
        (1, -1),
        (0, -1),
        (-1, -1),
        (17, -2),
        (14, -2),
        (11, -2),
        (8, -2),
        (5, -2),
        (2, -2),
    ],
];

// -----------------------------------------------------------------------------
// Rotated/flipped tile indices
// -----------------------------------------------------------------------------

const TILE_INDICES: [(i32, i32, i32); 8] = [
    // Up, Right
    (TILE_SIZE as i32, 1, TILE_SIZE as i32 + 1),
    // Up, Top
    (-1, TILE_SIZE as i32, (2 * TILE_SIZE) as i32 - 2),
    // Up, Left
    (
        -(TILE_SIZE as i32),
        -1,
        ((TILE_SIZE - 1) * TILE_SIZE) as i32 - 2,
    ),
    // Up, Bottom
    (
        1,
        -(TILE_SIZE as i32),
        ((TILE_SIZE - 2) * TILE_SIZE) as i32 + 1,
    ),
    // Down, Right
    (
        -(TILE_SIZE as i32),
        1,
        ((TILE_SIZE - 2) * TILE_SIZE) as i32 + 1,
    ),
    // Down, Top
    (
        -1,
        -(TILE_SIZE as i32),
        ((TILE_SIZE - 1) * TILE_SIZE) as i32 - 2,
    ),
    // Down, Left
    (TILE_SIZE as i32, -1, (2 * TILE_SIZE) as i32 - 2),
    // Down, Bottom
    (1, TILE_SIZE as i32, TILE_SIZE as i32 + 1),
];

// -----------------------------------------------------------------------------
