use crate::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

// -----------------------------------------------------------------------------
// Bag graph
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Holding {
    key: u32,
    number: usize,
}

#[derive(Debug)]
struct Node {
    contained_by: Vec<u32>,
    contains: Vec<Holding>,
}

fn calculate_hash(t: &str) -> u32 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish() as u32
}

fn add_to_graph(s: &str, bag_graph: &mut HashMap<u32, Node>) {
    let mut input = s.split(" bags contain ");
    let container_key = calculate_hash(input.next().unwrap());
    let mut contains = Vec::<Holding>::new();
    // Containing bags
    for line in input.next().unwrap().split(", ") {
        if line.as_bytes()[0] == b'n' {
            break;
        }
        let mut line = line.splitn(2, " ");
        let number: usize = line.next().unwrap().parse().unwrap();
        let contained_key = calculate_hash(line.next().unwrap().split(" bag").next().unwrap());
        bag_graph
            .entry(contained_key)
            .or_insert(Node {
                contained_by: Vec::<u32>::new(),
                contains: Vec::<Holding>::new(),
            })
            .contained_by
            .push(container_key);
        contains.push(Holding {
            key: contained_key,
            number: number,
        });
    }
    // Contained bags
    bag_graph
        .entry(container_key)
        .or_insert(Node {
            contained_by: Vec::<u32>::new(),
            contains: Vec::<Holding>::new(),
        })
        .contains
        .append(&mut contains);
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(key: u32, bag_graph: &HashMap<u32, Node>, mut uniques: &mut HashSet<u32>) -> usize {
    if !uniques.insert(key.clone()) {
        return 0;
    };
    match bag_graph.get(&key) {
        Some(node) => {
            1 + node
                .contained_by
                .iter()
                .map(|container| part_1(*container, &bag_graph, &mut uniques))
                .sum::<usize>()
        }
        None => 1,
    }
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(key: u32, bag_graph: &HashMap<u32, Node>) -> usize {
    match bag_graph.get(&key) {
        Some(node) => {
            1 + node
                .contains
                .iter()
                .map(|bag| bag.number * part_2(bag.key, &bag_graph))
                .sum::<usize>()
        }
        None => 1,
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
    let buffer: String = std::fs::read_to_string("data/day07.txt").unwrap();

    // Read to graph
    let mut bag_graph = HashMap::<u32, Node>::new();
    buffer
        .lines()
        .for_each(|line| add_to_graph(line, &mut bag_graph));
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_1 = Instant::now();
    let mut uniques = HashSet::<u32>::new();
    let count_1 = part_1(calculate_hash("shiny gold"), &bag_graph, &mut uniques) - 1;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_2 = Instant::now();
    let count_2 = part_2(calculate_hash("shiny gold"), &bag_graph) - 1;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part_1: count_1 as i64,
        part_2: count_2 as i64,
        times: Timing {
            setup: time_setup,
            part_1: time_part_1,
            part_2: time_part_2,
            combined: time_setup + time_part_1 + time_part_2,
        },
    };
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(7);
    output::print_part(1, "🧳 Containing", &format!("{}", results.part_1));
    output::print_part(2, "🧳 Contains", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
