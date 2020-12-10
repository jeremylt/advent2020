//! Day 7:
//! A particularly slow day, the performance is dominated by adding each bag to a graph.
//! Once the input has been parsed, traversing the graph to find the contained/containing
//! bags is straightforward and fast. Switching to the rustc hasher helped trim some time.

use crate::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::RwLock;

const CAPACITY: usize = 512;

// -----------------------------------------------------------------------------
// Bag graph
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Holding {
    key: u32,
    number: usize,
}

impl Holding {
    fn new(key: u32, number: usize) -> Self {
        Self { key, number }
    }
}

#[derive(Debug)]
struct Node {
    contained_by: Vec<u32>,
    contains: Vec<Holding>,
}

impl Node {
    fn new() -> Self {
        Self {
            contained_by: vec![],
            contains: vec![],
        }
    }
}

#[inline(always)]
fn str_to_key(t: &str) -> u32 {
    t.as_bytes()
        .iter()
        .fold(1 as u32, |acc, c| acc * 2 + *c as u32)
}

fn add_to_graph(s: &str, bag_graph: &mut FxHashMap<u32, Node>) {
    let mut input = s.splitn(2, " bags contain ");
    let container_str = input.next();
    let contents = input.next().unwrap();
    // Continue only if contents aren't 'no other bags'
    if contents.as_bytes()[0] == b'n' {
        return;
    }
    let container_key = str_to_key(container_str.unwrap());
    let mut contains = vec![];
    // Containing bags
    for line in contents.split(", ") {
        let number = line.chars().nth(0).unwrap().to_digit(10).unwrap() as usize; // Number is always one digit
        let contained_key = str_to_key(line[2..].splitn(2, " bag").next().unwrap());
        bag_graph
            .entry(contained_key)
            .or_insert(Node::new())
            .contained_by
            .push(container_key);
        contains.push(Holding::new(contained_key, number));
    }
    // Contained bags
    bag_graph
        .entry(container_key)
        .or_insert(Node::new())
        .contains = contains;
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
lazy_static! {
    static ref UNIQUES: RwLock<FxHashSet<u32>> = RwLock::new(
        FxHashSet::<u32>::with_capacity_and_hasher(CAPACITY, Default::default())
    );
}

fn part_1(key: u32, bag_graph: &FxHashMap<u32, Node>) -> usize {
    if !UNIQUES.write().unwrap().insert(key.clone()) {
        return 0;
    };
    match bag_graph.get(&key) {
        Some(node) => node
            .contained_by
            .iter()
            .fold(1, |acc, container| acc + part_1(*container, &bag_graph)),
        None => 1,
    }
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(key: u32, bag_graph: &FxHashMap<u32, Node>) -> usize {
    match bag_graph.get(&key) {
        Some(node) => node
            .contains
            .iter()
            .fold(1, |acc, bag| acc + bag.number * part_2(bag.key, &bag_graph)),
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
    let mut bag_graph =
        FxHashMap::<u32, Node>::with_capacity_and_hasher(CAPACITY, Default::default());
    buffer
        .lines()
        .for_each(|line| add_to_graph(line, &mut bag_graph));
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find number of containing bags
    let start_part_1 = Instant::now();
    let count_1 = part_1(str_to_key("shiny gold"), &bag_graph) as i64 - 1;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find number of contained bags
    let start_part_2 = Instant::now();
    let count_2 = part_2(str_to_key("shiny gold"), &bag_graph) - 1;
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
    output::print_day(7, "Handy Haversacks");
    output::print_part(1, "🧳 Containing", &format!("{}", results.part_1));
    output::print_part(2, "🧳 Contains", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
