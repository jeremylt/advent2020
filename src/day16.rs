//! Day 16:
//! The range checking is somewhat expensive in this problem. I imagine there is a better
//! way to organize the data for finding the rules, but I haven't put much thought into
//! it yet.

use crate::prelude::*;

// Constants
const NUMBER_FIELDS: usize = 20;

// -----------------------------------------------------------------------------
// Ticket fields
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct TicketField {
    name: String,
    lower_range: std::ops::RangeInclusive<u16>,
    upper_range: std::ops::RangeInclusive<u16>,
}

impl std::str::FromStr for TicketField {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Line of form FIELD NAME: LOWER-RANGE or UPPER-RANGE
        let mut line = s.splitn(2, ": ");
        let name = line.next().unwrap().to_string();

        let mut ranges = line.next().unwrap().splitn(4, &['-', 'o'][..]);
        let lower_range = // need to remove extra spaces
            ranges.next().unwrap().parse()?..=ranges.next().unwrap().trim().parse()?;
        let upper_range = // need to remove 'r ' and '\n'
            ranges.next().unwrap()[2..].parse()?..=ranges.next().unwrap().trim().parse()?;

        Ok(Self {
            name,
            lower_range,
            upper_range,
        })
    }
}

// -----------------------------------------------------------------------------
// Ticket field checker
// -----------------------------------------------------------------------------
#[inline]
fn valid_field(value: &u16, field: &TicketField) -> bool {
    field.upper_range.contains(value) || field.lower_range.contains(value)
}

#[inline]
fn valid_fields(value: &u16, fields: &Vec<TicketField>) -> bool {
    fields.iter().any(|field| valid_field(value, field))
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
    let buffer: String = std::fs::read_to_string("data/day16.txt").unwrap();

    // Read ticket fields
    let mut data = buffer.split("\n\n");
    let fields: Vec<TicketField> = data
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<TicketField>().expect("failed to parse field"))
        .collect();

    // My ticket
    let my_ticket: Vec<u16> = data
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|value| value.parse().expect("failed to parse data"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Check nearby tickets, filter out invalid
    let start_part_1 = Instant::now();
    let mut error_rate_1 = 0;
    let other_tickets: Vec<[u16; NUMBER_FIELDS]> = data
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut other_ticket = [0; NUMBER_FIELDS];
            let mut invalid_data = false;
            line.split(",").enumerate().for_each(|(i, raw)| {
                let value = raw.parse().expect("failed to parse data");
                if !valid_fields(&value, &fields) {
                    error_rate_1 += value;
                    invalid_data = true;
                } else {
                    other_ticket[i] = value;
                }
            });
            if invalid_data {
                // Strip tickets with invalid data
                None
            } else {
                Some(other_ticket)
            }
        })
        .collect();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Match fields to location
    let start_part_2 = Instant::now();
    let mut match_count = 0;
    let mut matches = [NUMBER_FIELDS + 1; NUMBER_FIELDS];
    let mut unmatched: Vec<Vec<usize>> = vec![(0..NUMBER_FIELDS).collect(); NUMBER_FIELDS];

    while match_count < NUMBER_FIELDS {
        other_tickets.iter().for_each(|ticket| {
            let mut found = NUMBER_FIELDS + 1;
            // Remove invalid options
            unmatched.iter_mut().enumerate().for_each(|(i, position)| {
                position.retain(|&possible| valid_field(&ticket[i], &fields[possible]));
                // Clear position if found
                if position.len() == 1 {
                    matches[i] = position[0];
                    found = position[0];
                    position.clear();
                }
            });
            // Remove index if found
            if found != NUMBER_FIELDS + 1 {
                unmatched
                    .iter_mut()
                    .for_each(|position| position.retain(|&value| value != found));
                match_count += 1;
            }
        });
        match_count = matches
            .iter()
            .filter(|&value| *value != NUMBER_FIELDS + 1)
            .count();
    }

    let product_2 = matches
        .iter()
        .enumerate()
        .fold(1, |acc, (match_index, &ticket_index)| {
            if fields[ticket_index].name[0..2] == *"de" {
                acc * my_ticket[match_index] as usize
            } else {
                acc
            }
        });
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        error_rate_1 as i64,
        product_2 as i64,
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
    output::print_day(1, "Report Repair");
    output::print_part(1, "🎫 Invalid", &format!("{}", results.part_1));
    output::print_part(2, "🎫 Product", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
