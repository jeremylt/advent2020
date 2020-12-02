use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub fn run() -> (usize, usize) {
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
            (?P<min>\d+) # min number of reps
            -
            (?P<max>\d+) # max number of reps
            \s
            (?P<required>\w): # required character
            \s
            (?P<password>\w+) # password to check
            ",
        )
        .unwrap();
    }
    for line in buffer.lines() {
        // Parse line
        let capture_groups = RE_LINE.captures(line.as_ref().unwrap()).unwrap();
        let min = capture_groups
            .name("min")
            .map_or("", |m| m.as_str())
            .parse()
            .unwrap();
        let max = capture_groups
            .name("max")
            .map_or("", |m| m.as_str())
            .parse()
            .unwrap();
        let required: String = (*capture_groups
            .name("required")
            .map_or("", |m| m.as_str())
            .clone())
        .to_string();
        let password: String = (*capture_groups
            .name("password")
            .map_or("", |m| m.as_str())
            .clone())
        .to_string();
        data.push((min, max, required, password));
    }

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    let mut part_1 = 0;
    for (min, max, required, password) in &data {
        // Check line
        let number_matches = password.matches(required).count();
        if (number_matches >= *min) && (number_matches <= *max) {
            part_1 += 1;
        }
    }

    // Report
    println!("    Part 1:");
    println!("      Number of Valid: {}", part_1);

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    let mut part_2 = 0;
    for (min, max, required, password) in &data {
        // Check line
        let length = password.len();
        if (length >= *min)
            && (length >= *max)
            && ((password.as_bytes()[min - 1] == required.as_bytes()[0])
                || (password.as_bytes()[max - 1] == required.as_bytes()[0]))
            && (password.as_bytes()[min - 1] != password.as_bytes()[max - 1])
        {
            part_2 += 1;
        }
    }
    // Report
    println!("    Part 2:");
    println!("      Number of Valid: {}", part_2);

    // Return
    return (part_1, part_2);
}

// -----------------------------------------------------------------------------
