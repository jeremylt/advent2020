use crate::prelude::*;
use regex::Regex;

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
pub(crate) fn run() -> Results {
    println!("- Day 2");
    let start_all = Instant::now();

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

    // Timing
    let time_setup = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_1 = Instant::now();
    let count_1 = data.iter().fold(0, part01);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_2 = Instant::now();
    let count_2 = data.iter().fold(0, part02);
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Timing
    // -------------------------------------------------------------------------
    let time = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Report
    // -------------------------------------------------------------------------
    // Setup
    println!("    {}:", "Setup".yellow());
    println!("      Time: {:?}", time_setup);
    // Part 1
    println!("    {}:", "Part 1".cyan());
    println!("      Rule : required number");
    println!("      Valid: {}", count_1);
    println!("      Time: {:?}", time_part_1);
    // Part 2
    println!("    {}:", "Part 2".blue());
    println!("      Rule : only one of two");
    println!("      Valid: {}", count_2);
    println!("      Time: {:?}", time_part_2);
    // Timing
    println!("    Final Time: {:?}", time);
    let part_1_portion = std::cmp::max(
        1,
        (NUMBER_DASHES as f64 * (time_part_1.as_nanos() as f64 / time.as_nanos() as f64)) as usize,
    );
    let part_2_portion = std::cmp::max(
        1,
        (NUMBER_DASHES as f64 * (time_part_2.as_nanos() as f64 / time.as_nanos() as f64)) as usize,
    );
    let setup_portion = NUMBER_DASHES - part_1_portion - part_2_portion;
    println!(
        "{}{}{}",
        "-".repeat(setup_portion).yellow().bold(),
        "-".repeat(part_1_portion).cyan().bold(),
        "-".repeat(part_2_portion).blue().bold()
    );

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part1: count_1,
        part2: count_2,
        time: time.as_millis(),
    };
}

// -----------------------------------------------------------------------------
