//! Day 19:
//! CYK is a good fit here, but with the size of the strings, the arrays are quite large,
//! as can be seen with the capacity listed below.

use crate::prelude::*;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

// -----------------------------------------------------------------------------
// Rules
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
enum Rule {
    Terminal(u8),      // id: "a", "b"
    Any(Vec<Vec<u8>>), // id: 1 | 2
    All(Vec<u8>),      // id: 1 2
}

fn check_rule<'a>(word: &'a str, rules: &FxHashMap<u8, Rule>, rule: &Rule) -> Result<&'a str, ()> {
    if word.is_empty() {
        return Err(());
    }
    match rule {
        // Character matches terminal rule
        Rule::Terminal(c) => {
            if word.as_bytes()[0] == *c {
                Ok(&word[1..])
            } else {
                Err(())
            }
        }
        // Any associated rule or set of rules matches
        Rule::Any(sub_rules) => {
            for option in sub_rules {
                let current = check_rule(word, rules, &Rule::All(option.clone()));
                if current.is_ok() {
                    return current;
                }
            }
            Err(())
        }
        // All rules match
        Rule::All(rule_indices) => {
            let mut remainder = word;
            for index in rule_indices {
                remainder = check_rule(remainder, rules, &rules[&index])?;
            }
            Ok(remainder)
        }
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
    let buffer: String = std::fs::read_to_string("data/day19.txt").unwrap();
    let mut data = buffer.split("\n\n");

    // Read to rules map
    let rules: FxHashMap<u8, Rule> = data
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut data = line.splitn(2, ": ");
            let index: u8 = data.next().unwrap().parse().unwrap();
            let rule: Rule;
            let right_side = data.next().unwrap();
            if &right_side[0..1] == "\"" {
                rule = Rule::Terminal(right_side.as_bytes()[1]);
            } else {
                let subs = right_side
                    .split(" | ")
                    .map(|sub| {
                        sub.split(" ")
                            .map(|c| c.parse().unwrap())
                            .collect::<Vec<u8>>()
                    })
                    .collect::<Vec<Vec<u8>>>();
                if subs.len() == 1 {
                    rule = Rule::All(subs[0].clone());
                } else {
                    rule = Rule::Any(subs);
                }
            }
            (index, rule)
        })
        .collect();
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
            let remainder = check_rule(message, &rules, &rules[&0]);
            if remainder.is_ok() && remainder.unwrap().is_empty() {
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
    let count_extra = invalid_messages
        .par_iter()
        .filter(|&message| {
            // Check rule 42
            let mut count_42 = 0;
            let mut remainder = message.as_str();
            let mut result = check_rule(&remainder, &rules, &rules[&42]);
            while let Ok(current) = result {
                count_42 += 1;
                remainder = current;
                result = check_rule(&remainder, &rules, &rules[&42]);
            }

            if count_42 < 2 {
                return false;
            }

            // Check rule 31
            let mut count_31 = 0;
            let mut result = check_rule(&remainder, &rules, &rules[&31]);
            while let Ok(current) = result {
                count_31 += 1;
                remainder = current;
                result = check_rule(&remainder, &rules, &rules[&31]);
            }

            remainder.is_empty() && count_31 > 0 && count_42 > count_31
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

    let rules: FxHashMap<u8, Rule> = data
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut data = line.splitn(2, ": ");
            let index: u8 = data.next().unwrap().parse().unwrap();
            let rule: Rule;
            let right_side = data.next().unwrap();
            if &right_side[0..1] == "\"" {
                rule = Rule::Terminal(right_side.as_bytes()[1]);
            } else {
                let subs = right_side
                    .split(" | ")
                    .map(|sub| {
                        sub.split(" ")
                            .map(|c| c.parse().unwrap())
                            .collect::<Vec<u8>>()
                    })
                    .collect::<Vec<Vec<u8>>>();
                if subs.len() == 1 {
                    rule = Rule::All(subs[0].clone());
                } else {
                    rule = Rule::Any(subs);
                }
            }
            (index, rule)
        })
        .collect();

    let (combined_1, combined_2) = data
        .next()
        .unwrap()
        .par_lines()
        .map(|line| {
            let message = line.clone();

            let remainder = check_rule(message, &rules, &rules[&0]);
            let part_1 = remainder.is_ok() && remainder.unwrap().is_empty();

            let part_2 = part_1 || {
                // Check rule 42
                let mut count_42 = 0;
                let mut remainder = message;
                let mut result = check_rule(&remainder, &rules, &rules[&42]);
                while let Ok(current) = result {
                    count_42 += 1;
                    remainder = current;
                    result = check_rule(&remainder, &rules, &rules[&42]);
                }

                // Check rule 31
                let mut count_31 = 0;
                let mut result = check_rule(&remainder, &rules, &rules[&31]);
                while let Ok(current) = result {
                    count_31 += 1;
                    remainder = current;
                    result = check_rule(&remainder, &rules, &rules[&31]);
                }

                remainder.is_empty() && count_42 >= 2 && count_31 > 0 && count_42 > count_31
            };
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
// Cocke Younger Kasami
// -----------------------------------------------------------------------------
/*macro_rules! index_3d {
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
}*/

// -----------------------------------------------------------------------------
