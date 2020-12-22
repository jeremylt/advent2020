//! Day 22:

use crate::prelude::*;
use arrayvec::ArrayVec;
use rustc_hash::FxHashSet;

// Constant
const DECK_SIZE: usize = 50;
const CAPACITY: usize = 64;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
#[inline]
fn score_game(head: usize, deck: &ArrayVec<[u8; DECK_SIZE]>) -> usize {
    let offset = DECK_SIZE - head % DECK_SIZE;
    deck.iter()
        .enumerate()
        .map(|(i, &card)| card as usize * (DECK_SIZE - (i + offset) % DECK_SIZE))
        .sum()
}

#[derive(Debug, PartialEq)]
enum Winner {
    Player1,
    Player2,
}

#[inline]
fn part_1(
    mut player_1_head: usize,
    mut player_1_tail: usize,
    mut player_1_deck: ArrayVec<[u8; DECK_SIZE]>,
    mut player_2_head: usize,
    mut player_2_tail: usize,
    mut player_2_deck: ArrayVec<[u8; DECK_SIZE]>,
) -> (Winner, usize) {
    // Play game
    while player_1_head != player_1_tail && player_2_head != player_2_tail {
        let card_1 = player_1_deck[player_1_head % DECK_SIZE];
        player_1_head += 1;
        let card_2 = player_2_deck[player_2_head % DECK_SIZE];
        player_2_head += 1;

        if card_1 > card_2 {
            player_1_deck[player_1_tail % DECK_SIZE] = card_1;
            player_1_tail += 1;
            player_1_deck[player_1_tail % DECK_SIZE] = card_2;
            player_1_tail += 1;
        } else {
            player_2_deck[player_2_tail % DECK_SIZE] = card_2;
            player_2_tail += 1;
            player_2_deck[player_2_tail % DECK_SIZE] = card_1;
            player_2_tail += 1;
        }
    }

    // Report winner
    if player_2_head == player_2_tail {
        (Winner::Player1, score_game(player_1_head, &player_1_deck))
    } else {
        (Winner::Player2, score_game(player_2_head, &player_2_deck))
    }
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn hash_two(first: u64, second: u64) -> u64 {
    first ^ (second + (first << 6) + (first >> 2))
}

fn part_2(
    mut player_1_head: usize,
    mut player_1_tail: usize,
    mut player_1_deck: ArrayVec<[u8; DECK_SIZE]>,
    mut player_2_head: usize,
    mut player_2_tail: usize,
    mut player_2_deck: ArrayVec<[u8; DECK_SIZE]>,
    score: bool,
) -> (Winner, usize) {
    // Short circuit sub game if Player 1 has largest card
    if !score {
        let player_1_max = (player_1_head..player_1_tail)
            .map(|i| player_1_deck[i % DECK_SIZE])
            .fold(0, |acc, elem| std::cmp::max(acc, elem));
        if (player_2_head..player_2_tail).all(|i| player_2_deck[i % DECK_SIZE] < player_1_max) {
            return (Winner::Player1, 0);
        }
    }

    // Visited configurations
    let mut visited_configurations =
        FxHashSet::<u64>::with_capacity_and_hasher(CAPACITY, Default::default());
    let hash_player_1 = (player_1_head..player_1_tail).fold(0, |acc, i| {
        hash_two(acc, player_1_deck[i % DECK_SIZE] as u64)
    });
    let hash_player_2 = (player_2_head..player_2_tail).fold(0, |acc, i| {
        hash_two(acc, player_2_deck[i % DECK_SIZE] as u64)
    });
    visited_configurations.insert(hash_two(hash_player_1, hash_player_2));

    // Play game
    while player_1_head != player_1_tail && player_2_head != player_2_tail {
        // Draw cards
        let card_1 = player_1_deck[player_1_head % DECK_SIZE];
        player_1_head += 1;
        let card_2 = player_2_deck[player_2_head % DECK_SIZE];
        player_2_head += 1;

        let winner: Winner;
        // Recursive round
        if card_1 <= (player_1_tail - player_1_head) as u8
            && card_2 <= (player_2_tail - player_2_head) as u8
        {
            let (winner_, _) = part_2(
                player_1_head,
                player_1_head + card_1 as usize,
                player_1_deck.clone(),
                player_2_head,
                player_2_head + card_2 as usize,
                player_2_deck.clone(),
                false,
            );
            winner = winner_; // Ugly hack because destructuring is unstable
        } else if card_1 > card_2 {
            // Standard round
            winner = Winner::Player1;
        } else {
            winner = Winner::Player2;
        }

        if winner == Winner::Player1 {
            player_1_deck[player_1_tail % DECK_SIZE] = card_1;
            player_1_tail += 1;
            player_1_deck[player_1_tail % DECK_SIZE] = card_2;
            player_1_tail += 1;
        } else {
            player_2_deck[player_2_tail % DECK_SIZE] = card_2;
            player_2_tail += 1;
            player_2_deck[player_2_tail % DECK_SIZE] = card_1;
            player_2_tail += 1;
        }

        // Check for visited configuration
        let hash_player_1 = (player_1_head..player_1_tail).fold(0, |acc, i| {
            hash_two(acc, player_1_deck[i % DECK_SIZE] as u64)
        });
        let hash_player_2 = (player_2_head..player_2_tail).fold(0, |acc, i| {
            hash_two(acc, player_2_deck[i % DECK_SIZE] as u64)
        });
        if !visited_configurations.insert(hash_two(hash_player_1, hash_player_2)) {
            return (
                Winner::Player1,
                if score {
                    score_game(player_1_head, &player_1_deck)
                } else {
                    0
                },
            );
        }
    }

    // Report winner
    if player_2_head == player_2_tail {
        (
            Winner::Player1,
            if score {
                score_game(player_1_head, &player_1_deck)
            } else {
                0
            },
        )
    } else {
        (
            Winner::Player2,
            if score {
                score_game(player_2_head, &player_2_deck)
            } else {
                0
            },
        )
    }
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
    let buffer: String = std::fs::read_to_string("data/day22.txt").unwrap();
    let mut data = buffer.split("\n\n");

    // Read to arrays
    let mut player_1_deck: ArrayVec<[u8; DECK_SIZE]> = ArrayVec::from([0; DECK_SIZE]);
    let player_1_head = 0;
    let mut player_1_tail = 0;
    data.next().unwrap().lines().skip(1).for_each(|value| {
        player_1_deck[player_1_tail] = value.parse().expect("failed to parse line");
        player_1_tail += 1;
    });

    let mut player_2_deck: ArrayVec<[u8; DECK_SIZE]> = ArrayVec::from([0; DECK_SIZE]);
    let player_2_head = 0;
    let mut player_2_tail = 0;
    data.next().unwrap().lines().skip(1).for_each(|value| {
        player_2_deck[player_2_tail] = value.parse().expect("failed to parse line");
        player_2_tail += 1;
    });

    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Play simple game
    let start_part_1 = Instant::now();
    let (_, score_1) = part_1(
        player_1_head,
        player_1_tail,
        player_1_deck.clone(),
        player_2_head,
        player_2_tail,
        player_2_deck.clone(),
    );
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Play recursive game
    let start_part_2 = Instant::now();
    let (_, score_2) = part_2(
        player_1_head,
        player_1_tail,
        player_1_deck.clone(),
        player_2_head,
        player_2_tail,
        player_2_deck.clone(),
        true,
    );
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        score_1 as i64,
        score_2 as i64,
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
    output::print_day(22, "Crab Combat");
    output::print_part(1, "🦀 Score", &format!("{}", results.part_1));
    output::print_part(2, "🦀 Score", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
