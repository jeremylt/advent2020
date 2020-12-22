//! Day 19:
//! CYK is a good fit here, but with the size of the strings, the arrays are quite large,
//! as can be seen with the capacity listed below.

use crate::prelude::*;
use rayon::prelude::*;

// Constants
const CAPACITY: usize = 512;

// -----------------------------------------------------------------------------
// Rules
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct TerminalRule {
    left: u8,
    symbol: char,
}

#[derive(Debug, Clone)]
struct ProductionRule {
    left: u8,
    first: u8,
    second: u8,
}

#[derive(Debug)]
struct UnitRule {
    left: u8,
    only: u8,
}

// -----------------------------------------------------------------------------
// Cocke Younger Kasami
// -----------------------------------------------------------------------------
macro_rules! index_3d {
    ($i:expr, $j:expr, $k:expr, $message_length:expr, $number_rules:expr) => {
        (($message_length) * (($message_length) + 1)
            - ($message_length - ($i)) * (($message_length) + 1 - ($i)))
            / 2
            * ($number_rules)
            + ($j) * ($number_rules)
            + $k
    };
}

#[inline]
fn cocke_younger_kasami(
    message: &str,
    number_rules: usize,
    terminal_rules: &Vec<TerminalRule>,
    production_rules: &Vec<ProductionRule>,
    unit_rules: &Vec<UnitRule>,
) -> bool {
    let message_length = message.len();
    // Initialize first pass
    let mut table = vec![false; message_length * (message_length + 1) / 2 * number_rules];
    message.chars().enumerate().for_each(|(i, c)| {
        terminal_rules.iter().any(|terminal| {
            if c == terminal.symbol {
                table[index_3d!(0, i, terminal.left as usize, message_length, number_rules)] = true;
                true
            } else {
                false
            }
        });
        unit_rules.iter().for_each(|unit| {
            table[index_3d!(0, i, unit.left as usize, message_length, number_rules)] |=
                table[index_3d!(0, i, unit.only as usize, message_length, number_rules)]
        });
    });
    // Apply CYK
    // Span length
    (1..message_length).for_each(|l| {
        // Span start
        (0..message_length - l).for_each(|s| {
            // Span partition
            (0..l).for_each(|p| {
                production_rules.iter().for_each(|production| {
                    if table[index_3d!(
                        p,
                        s,
                        production.first as usize,
                        message_length,
                        number_rules
                    )] && table[index_3d!(
                        l - p - 1,
                        s + p + 1,
                        production.second as usize,
                        message_length,
                        number_rules
                    )] {
                        table[index_3d!(
                            l,
                            s,
                            production.left as usize,
                            message_length,
                            number_rules
                        )] = true;
                    }
                });
                unit_rules.iter().for_each(|unit| {
                    table[index_3d!(l, s, unit.left as usize, message_length, number_rules)] |=
                        table[index_3d!(l, s, unit.only as usize, message_length, number_rules)]
                });
            })
        });
    });
    table[index_3d!(message_length - 1, 0, 0, message_length, number_rules)]
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
    let buffer: String = std::fs::read_to_string("data/day19.txt").unwrap();
    let mut data = buffer.split("\n\n");

    // Read to vector
    let mut production_rules: Vec<ProductionRule> = Vec::with_capacity(CAPACITY);
    let mut terminal_rules: Vec<TerminalRule> = Vec::with_capacity(CAPACITY);
    let mut unit_rules: Vec<UnitRule> = Vec::with_capacity(CAPACITY);
    let mut number_rules = 0;
    data.next().unwrap().lines().for_each(|line| {
        let mut data = line.splitn(2, ": ");
        let index: u8 = data.next().unwrap().parse().unwrap();
        let rule = data.next().unwrap();
        if &rule[0..1] == "\"" {
            terminal_rules.push(TerminalRule {
                left: index,
                symbol: rule.chars().nth(1).unwrap(),
            });
        } else {
            rule.split(" | ").for_each(|sub| {
                let symbols: Vec<u8> = sub.split(" ").map(|c| c.parse().unwrap()).collect();
                match symbols.len() {
                    1 => unit_rules.push(UnitRule {
                        left: index,
                        only: symbols[0],
                    }),
                    2 => production_rules.push(ProductionRule {
                        left: index,
                        first: symbols[0],
                        second: symbols[1],
                    }),
                    _ => panic!("unmatched rule"),
                }
            });
        }
        number_rules = std::cmp::max(number_rules, index + 1);
    });
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Count valid messages
    let start_part_1 = Instant::now();
    let messages: Vec<String> = data
        .next()
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let invalid_messages: Vec<String> = messages
        .par_iter()
        .filter_map(|message| {
            if cocke_younger_kasami(
                message,
                number_rules as usize,
                &terminal_rules,
                &production_rules,
                &unit_rules,
            ) {
                None
            } else {
                Some(message.to_string())
            }
        })
        .collect();
    let count_1 =
        buffer.split("\n\n").skip(1).next().unwrap().lines().count() - invalid_messages.len();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Count additional messages
    let start_part_2 = Instant::now();
    production_rules.push(ProductionRule {
        left: 42,
        first: 42,
        second: 42,
    });
    production_rules.push(ProductionRule {
        left: 11,
        first: 42,
        second: number_rules,
    });
    production_rules.push(ProductionRule {
        left: number_rules,
        first: 11,
        second: 31,
    });
    number_rules += 1;
    let count_extra = invalid_messages
        .par_iter()
        .filter(|message| {
            cocke_younger_kasami(
                message,
                number_rules as usize + 1,
                &terminal_rules,
                &production_rules,
                &unit_rules,
            )
        })
        .count();
    let count_2 = count_1 + count_extra;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let buffer: String = std::fs::read_to_string("data/day19.txt").unwrap();
    let mut data = buffer.split("\n\n");

    let mut production_rules: Vec<ProductionRule> = Vec::with_capacity(CAPACITY);
    let mut terminal_rules: Vec<TerminalRule> = Vec::with_capacity(CAPACITY);
    let mut unit_rules: Vec<UnitRule> = Vec::with_capacity(CAPACITY);
    let mut number_rules = 0;
    data.next().unwrap().lines().for_each(|line| {
        let mut data = line.splitn(2, ": ");
        let index: u8 = data.next().unwrap().parse().unwrap();
        let rule = data.next().unwrap();
        if &rule[0..1] == "\"" {
            terminal_rules.push(TerminalRule {
                left: index,
                symbol: rule.chars().nth(1).unwrap(),
            });
        } else {
            rule.split(" | ").for_each(|sub| {
                let symbols: Vec<u8> = sub.split(" ").map(|c| c.parse().unwrap()).collect();
                match symbols.len() {
                    1 => unit_rules.push(UnitRule {
                        left: index,
                        only: symbols[0],
                    }),
                    2 => production_rules.push(ProductionRule {
                        left: index,
                        first: symbols[0],
                        second: symbols[1],
                    }),
                    _ => panic!("unmatched rule"),
                }
            });
        }
        number_rules = std::cmp::max(number_rules, index + 1);
    });

    let mut production_rules_part_2 = production_rules.clone();
    production_rules_part_2.push(ProductionRule {
        left: 42,
        first: 42,
        second: 42,
    });
    production_rules_part_2.push(ProductionRule {
        left: 11,
        first: 42,
        second: number_rules,
    });
    production_rules_part_2.push(ProductionRule {
        left: number_rules,
        first: 11,
        second: 31,
    });

    let (combined_1, combined_2) = data
        .next()
        .unwrap()
        .par_lines()
        .map(|line| {
            let message = line.to_string();
            let part_1 = cocke_younger_kasami(
                &message,
                number_rules as usize,
                &terminal_rules,
                &production_rules,
                &unit_rules,
            );
            let part_2 = part_1
                || cocke_younger_kasami(
                    &message,
                    number_rules as usize + 1,
                    &terminal_rules,
                    &production_rules_part_2,
                    &unit_rules,
                );
            (part_1 as usize, part_2 as usize)
        })
        .reduce(
            || (0, 0),
            |acc, partial| (acc.0 + partial.0, acc.1 + partial.1),
        );
    let time_combined = start_combined.elapsed();
    assert_eq!(count_1, combined_1);
    assert_eq!(count_2, combined_2);

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        count_1 as i64,
        count_2 as i64,
        Timing::new(time_setup, time_part_1, time_part_2, time_combined),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(19, "Monster Messages");
    output::print_part(1, "🦕 Valid", &format!("{}", results.part_1));
    output::print_part(2, "🦕 Valid", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
