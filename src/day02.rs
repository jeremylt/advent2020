use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part01(acc: i32, (upper, lower, required, password): &(usize, usize, String, String)) -> i32 {
    let number_matches = password.matches(required).count();
    if (number_matches >= *upper) && (number_matches <= *lower) {
        acc + 1
    } else {
        acc
    }
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part02(acc: i32, (upper, lower, required, password): &(usize, usize, String, String)) -> i32 {
    let first = &password[lower - 1..*lower];
    let second = &password[upper - 1..*upper];
    if (first != second) && ((first == required) || (second == required)) {
        acc + 1
    } else {
        acc
    }
}

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub fn run() -> (i32, i32) {
    println!("\n- Day 2:");

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
    let mut data: Vec<(usize, usize, String, String)> = Vec::new();
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
        data.push((lower, upper, required, password));
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

    // Return
    return (count_1, count_2);
}

// -----------------------------------------------------------------------------
