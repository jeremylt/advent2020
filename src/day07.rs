use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

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

fn str_to_key(t: &str) -> u32 {
    t.as_bytes()
        .iter()
        .fold(1 as u32, |acc, c| acc * 2 + *c as u32)
}

fn add_to_graph(s: &str, bag_graph: &mut HashMap<u32, Node>) {
    let mut input = s.splitn(2, " bags contain ");
    let container_key = str_to_key(input.next().unwrap());
    let mut contains = vec![];
    // Containing bags
    for line in input.next().unwrap().split(", ") {
        let mut line = line.splitn(2, " ");
        let number: usize;
        match line.next().unwrap().parse() {
            Ok(val) => number = val,
            Err(_) => break,
        }
        let contained_key = str_to_key(line.next().unwrap().splitn(2, " bag").next().unwrap());
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
    static ref UNIQUES: RwLock<HashSet<u32>> = RwLock::new(HashSet::<u32>::with_capacity(524));
}

fn part_1(key: u32, bag_graph: &HashMap<u32, Node>) -> usize {
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
fn part_2(key: u32, bag_graph: &HashMap<u32, Node>) -> usize {
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
    let mut bag_graph = HashMap::<u32, Node>::with_capacity(524);
    buffer
        .lines()
        .for_each(|line| add_to_graph(line, &mut bag_graph));
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_1 = Instant::now();
    let count_1 = part_1(str_to_key("shiny gold"), &bag_graph) as i64 - 1;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passwords
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
            time_setup + time_part_1 + time_part_2,
        ),
    )
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
