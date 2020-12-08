use crate::prelude::*;
use std::collections::{HashMap, HashSet};

// -----------------------------------------------------------------------------
// Instructions
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct Node {
    instruction: Instruction,
    value: i32,
    increment: i32,
    next: i32,
}

impl Node {
    fn new(s: &str, i: i32) -> Self {
        let mut input = s.splitn(2, " ");
        let instruction_char = input.next().unwrap().chars().next().unwrap();
        let value = input.next().unwrap().parse().unwrap();
        let instruction: Instruction;
        let increment: i32;
        let next: i32;
        match instruction_char {
            'a' => {
                instruction = Instruction::Acc;
                increment = value;
                next = i + 1
            }
            'j' => {
                instruction = Instruction::Jmp;
                increment = 0;
                next = i + value
            }
            'n' => {
                instruction = Instruction::Nop;
                increment = 0;
                next = i + 1
            }
            _ => panic!("Unknown instruction"),
        }
        Self {
            instruction,
            value,
            increment,
            next,
        }
    }
}

// -----------------------------------------------------------------------------
// State
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct State {
    count: i32,
    current: i32,
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(
    start: i32,
    number_instructions: i32,
    instructions: &HashMap<i32, Node>,
    previous_executed: &HashSet<i32>,
) -> (bool, i32) {
    let mut executed = HashSet::<i32>::with_capacity(256);
    let mut count = 0;
    let mut current = start;
    while executed.insert(current) && !previous_executed.contains(&current) {
        let node = instructions.get(&current).unwrap();
        current = node.next;
        count += node.increment;
        if current >= number_instructions {
            return (true, count);
        }
    }
    (false, count)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(number_instructions: i32, instructions: &HashMap<i32, Node>) -> i32 {
    let mut machine = Vec::<State>::with_capacity(256);
    let mut executed = HashSet::<i32>::with_capacity(256);
    let mut count = 0;
    let mut current = 0;
    while executed.insert(current) {
        machine.push(State { count, current });
        let node = instructions.get(&current).unwrap();
        current = node.next;
        count += node.increment;
    }
    machine
        .iter()
        .rev()
        .find_map(|state| {
            executed.remove(&state.current);
            let node = instructions.get(&state.current).unwrap();
            match node.instruction {
                Instruction::Acc => None,
                Instruction::Jmp => {
                    let (terminated, updated_count) = part_1(
                        state.current + 1,
                        number_instructions,
                        &instructions,
                        &executed,
                    );
                    if terminated {
                        Some(state.count + updated_count)
                    } else {
                        None
                    }
                }
                Instruction::Nop => {
                    let (terminated, updated_count) = part_1(
                        state.current + node.value,
                        number_instructions,
                        &instructions,
                        &executed,
                    );
                    if terminated {
                        Some(state.count + updated_count)
                    } else {
                        None
                    }
                }
            }
        })
        .unwrap()
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
    let buffer: String = std::fs::read_to_string("data/day08.txt").unwrap();

    // Read to graph
    let instructions: HashMap<i32, Node> = buffer
        .lines()
        .enumerate()
        .map(|(i, line)| (i as i32, Node::new(line, i as i32)))
        .collect();
    let number_instructions = instructions.len() as i32;
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find first repeated instruction
    let start_part_1 = Instant::now();
    let (_, count_1) = part_1(
        0,
        number_instructions,
        &instructions,
        &HashSet::<i32>::new(),
    );
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_2 = Instant::now();
    let count_2 = part_2(number_instructions, &instructions);
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
    output::print_day(8);
    output::print_part(1, "🎮 Infinite", &format!("{}", results.part_1));
    output::print_part(2, "🎮 Corrected", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
