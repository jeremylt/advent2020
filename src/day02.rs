use crate::prelude::*;

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

impl std::str::FromStr for PasswordData {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = s.split(|c| "- :".contains(c));
        let lower = m.next().unwrap().parse()?;
        let upper = m.next().unwrap().parse()?;
        let required = m.next().unwrap().to_string();
        assert_eq!(m.next(), Some(""));
        let password = m.next().unwrap().to_string();
        Ok(Self {
            lower,
            upper,
            required,
            password,
        })
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part01(data: &PasswordData) -> bool {
    let number_matches = data.password.matches(&data.required).count();
    (number_matches >= data.lower) && (number_matches <= data.upper)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part02(data: &PasswordData) -> bool {
    let first = &data.password[data.lower - 1..data.lower];
    let second = &data.password[data.upper - 1..data.upper];
    (first == data.required) != (second == data.required)
}

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub(crate) fn run() -> Results {
    println!("- {}", "Day 02".bold());
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

    // Read to object iterator
    let data = buffer.lines().map(|line| {
        line.unwrap()
            .parse::<PasswordData>()
            .expect("Could not parse line")
    });

    // Collect as a Vec so we can traverse multiple times (skip if only one traversal is needed)
    let data: Vec<_> = data.collect();

    // Timing
    let time_setup = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_1 = Instant::now();
    let count_1 = data.iter().filter(|&d| part01(d)).count();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_2 = Instant::now();
    let count_2 = data.iter().filter(|&d| part02(d)).count();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Timing
    // -------------------------------------------------------------------------
    let time = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Report
    // -------------------------------------------------------------------------
    // Setup
    println!("    {}:", "Setup".cyan().bold());
    println!("      Time     : {:?}", time_setup);
    // Part 1
    println!("    {}:", "Part 1".yellow().bold());
    println!("      Rule     : required number");
    println!("      Valid    : {}", count_1);
    println!("      Time     : {:?}", time_part_1);
    // Part 2
    println!("    {}:", "Part 2".blue().bold());
    println!("      Rule     : only one of two");
    println!("      Valid    : {}", count_2);
    println!("      Time     : {:?}", time_part_2);
    // Timing
    println!("    {} : {:?}", "Total Time".purple().bold(), time);
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
        "-".repeat(setup_portion).cyan().bold(),
        "-".repeat(part_1_portion).yellow().bold(),
        "-".repeat(part_2_portion).blue().bold()
    );

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part1: count_1 as i32,
        part2: count_2 as i32,
        time: time.as_nanos(),
    };
}

// -----------------------------------------------------------------------------
