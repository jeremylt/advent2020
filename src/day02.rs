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
            (?P<lower>\d+) # lower requirement
            -
            (?P<upper>\d+) # upper requirement
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
        let lower = capture_groups
            .name("lower")
            .map_or(0, |lower| lower.as_str().parse().unwrap());
        let upper = capture_groups
            .name("upper")
            .map_or(0, |upper| upper.as_str().parse().unwrap());
        let required: String = (*capture_groups
            .name("required")
            .map_or("", |required| required.as_str()))
        .to_string();
        let password: String = (*capture_groups
            .name("password")
            .map_or("", |password| password.as_str()))
        .to_string();
        data.push((lower, upper, required, password));
    }

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    let part_1 = data.iter().fold(0, |acc, (min, max, required, password)| {
        let number_matches = password.matches(required).count();
        if (number_matches >= *min) && (number_matches <= *max) {
            acc + 1
        } else {
            acc
        }
    });

    // Report
    println!("    Part 1:");
    println!("      Number of Valid: {}", part_1);

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    let part_2 = data
        .iter()
        .fold(0, |acc, (lower, upper, required, password)| {
            let first = &password[lower - 1..*lower];
            let second = &password[upper - 1..*upper];
            if (first != second) && ((first == required) || (second == required)) {
                acc + 1
            } else {
                acc
            }
        });

    // Report
    println!("    Part 2:");
    println!("      Number of Valid: {}", part_2);

    // Return
    return (part_1, part_2);
}

// -----------------------------------------------------------------------------
