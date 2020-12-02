use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

// -----------------------------------------------------------------------------
// Line parsing regex
// -----------------------------------------------------------------------------
lazy_static! {
    static ref RE_LINE: Regex = Regex::new(
        r"(?x)
            (?P<lower>\d+) # lower requirement
            -
            (?P<upper>\d+) # upper requirement
            \s
            (?P<required>\w) # required character
            :\s
            (?P<password>\w+) # password to check
            ",
    )
    .unwrap();
}

// -----------------------------------------------------------------------------
// Password data struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct PasswordData {
    lower: usize,
    upper: usize,
    required: String,
    password: String,
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part01(acc: i32, data: &PasswordData) -> i32 {
    let number_matches = data.password.matches(&data.required).count();
    if (number_matches >= data.lower) && (number_matches <= data.upper) {
        acc + 1
    } else {
        acc
    }
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part02(acc: i32, data: &PasswordData) -> i32 {
    let first = &data.password[data.lower - 1..data.lower];
    let second = &data.password[data.upper - 1..data.upper];
    if (first == data.required) != (second == data.required) {
        acc + 1
    } else {
        acc
    }
}

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub fn run() -> (i32, i32, u128) {
    println!("- Day 2");
    let start = Instant::now();

    // -------------------------------------------------------------------------
    // Data
    // -------------------------------------------------------------------------
    // Open file
    let path = "data/day02part01.txt";
    let input: File;
    match File::open(path) {
        Ok(file) => input = file,
        Err(_error) => panic!("Unable to open input file"),
    }
    let buffer = BufReader::new(input);

    // Read to vector
    let mut data: Vec<PasswordData> = Vec::new();
    for line in buffer.lines() {
        // Parse line
        let capture_groups = RE_LINE.captures(line.as_ref().unwrap()).unwrap();
        let lower = capture_groups
            .name("lower")
            .map_or(0, |value| value.as_str().parse().unwrap());
        let upper = capture_groups
            .name("upper")
            .map_or(0, |value| value.as_str().parse().unwrap());
        let required: String = (*capture_groups
            .name("required")
            .map_or("", |value| value.as_str()))
        .to_string();
        let password: String = (*capture_groups
            .name("password")
            .map_or("", |value| value.as_str()))
        .to_string();
        data.push(PasswordData {
            lower,
            upper,
            required,
            password,
        });
    }

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    let count_1 = data.iter().fold(0, part01);

    // Report
    println!("    Part 1:");
    println!("      Number of Valid: {}", count_1);

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    let count_2 = data.iter().fold(0, part02);

    // Report
    println!("    Part 2:");
    println!("      Number of Valid: {}", count_2);

    // Timing
    let time = start.elapsed();
    println!("    Time: {:?}", time);

    // Return
    return (count_1, count_2, time.as_millis());
}

// -----------------------------------------------------------------------------
