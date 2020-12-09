use crate::prelude::*;

const CAPACITY: usize = 512;

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
        // Instructions of the form
        //   acc value
        //   jmp value
        //   nop value
        let instruction_char = s.as_bytes()[0];
        let value = s[4..].parse::<i32>().unwrap();
        match instruction_char {
            b'a' => Self {
                instruction: Instruction::Acc,
                value: value,
                increment: value,
                next: i + 1,
            },
            b'j' => Self {
                instruction: Instruction::Jmp,
                value: value,
                increment: 0,
                next: i + value,
            },
            b'n' => Self {
                instruction: Instruction::Nop,
                value: value,
                increment: 0,
                next: i + 1,
            },
            _ => panic!("Unknown instruction"),
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
    let mut executed = previous_executed.clone();
    let mut count = 0;
    let mut current = start;
    // Iterate until repeat or out of bounds
    while executed.insert(current) && (current < number_instructions) {
        let node = instructions.get(&current).unwrap();
        current = node.next;
        count += node.increment;
    }
    (current >= number_instructions, count)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(number_instructions: i32, instructions: &HashMap<i32, Node>) -> i32 {
    let mut machine = Vec::<State>::with_capacity(CAPACITY);
    let mut executed = HashSet::<i32>::with_capacity(CAPACITY);
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
// Combined
// -----------------------------------------------------------------------------
fn run_program(
    start: i32,
    number_instructions: i32,
    instructions: &HashMap<i32, Node>,
    executed: &HashSet<i32>,
) -> (bool, i32) {
    let mut count = 0;
    let mut current = start;
    // Iterate until repeat or out of bounds
    while !executed.contains(&current) && (current < number_instructions) {
        let node = instructions.get(&current).unwrap();
        current = node.next;
        count += node.increment;
    }
    (current >= number_instructions, count)
}

fn combined(number_instructions: i32, instructions: &HashMap<i32, Node>) -> (i32, i32) {
    let mut machine = Vec::<State>::with_capacity(CAPACITY);
    let mut executed = HashSet::<i32>::with_capacity(CAPACITY);
    let mut count = 0;
    let mut current = 0;
    while executed.insert(current) {
        machine.push(State { count, current });
        let node = instructions.get(&current).unwrap();
        current = node.next;
        count += node.increment;
    }
    let count_1 = count;
    let count_2 = machine
        .iter()
        .rev()
        .find_map(|state| {
            let node = instructions.get(&state.current).unwrap();
            match node.instruction {
                Instruction::Acc => None,
                Instruction::Jmp => {
                    let (terminated, updated_count) = run_program(
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
                    let (terminated, updated_count) = run_program(
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
        .unwrap();
    (count_1, count_2)
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
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let instructions: HashMap<i32, Node> = buffer
        .lines()
        .enumerate()
        .map(|(i, line)| (i as i32, Node::new(line, i as i32)))
        .collect();
    let number_instructions = instructions.len() as i32;

    let (combined_1, combined_2) = combined(number_instructions, &instructions);
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, count_1);
    assert_eq!(combined_2, count_2);

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
    output::print_day(8);
    output::print_part(1, "🎮 Infinite", &format!("{}", results.part_1));
    output::print_part(2, "🎮 Corrected", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
