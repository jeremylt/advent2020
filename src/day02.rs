use crate::prelude::*;

// -----------------------------------------------------------------------------
// Password data struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct PasswordData {
    lower: usize,
    upper: usize,
    required: char,
    password: String,
}

impl std::str::FromStr for PasswordData {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = s.split(|c| "- :".contains(c));
        let lower = m.next().unwrap().parse()?;
        let upper = m.next().unwrap().parse()?;
        let required = m.next().unwrap().chars().next().unwrap();
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
fn part_1(data: &PasswordData) -> bool {
    let number_matches = data.password.matches(data.required).count();
    (number_matches >= data.lower) && (number_matches <= data.upper)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(data: &PasswordData) -> bool {
    let mut chars = data.password.chars();
    let first = chars.nth(data.lower - 1).unwrap();
    let second = chars.nth(data.upper - data.lower - 1).unwrap();
    (first == data.required) != (second == data.required)
}

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    if print_summary {
        println!("- {}", "Day 02".bold());
    }
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Data
    // -------------------------------------------------------------------------
    // Open file
    let buffer = file::load_file("data/day02part01.txt");

    // Read to object iterator
    let data: Vec<PasswordData> = buffer
        .lines()
        .map(|line| {
            line.unwrap()
                .parse::<PasswordData>()
                .expect("Could not parse line")
        })
        .collect();

    // Timing
    let time_setup = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_1 = Instant::now();
    let count_1 = data.iter().filter(|&d| part_1(d)).count();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passwords
    let start_part_2 = Instant::now();
    let count_2 = data.iter().filter(|&d| part_2(d)).count();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Timing
    // -------------------------------------------------------------------------
    let time = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Report
    // -------------------------------------------------------------------------
    if print_summary {
        output::print_setup(time_setup);
        output::print_part(
            1,
            "Rule",
            "required number",
            "Valid",
            &format!("{}", count_1),
            time_part_1,
        );
        output::print_part(
            2,
            "Rule",
            "only one of two",
            "Valid",
            &format!("{}", count_2),
            time_part_2,
        );
        output::print_timing(time, time_part_1, time_part_2);
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part1: count_1 as i64,
        part2: count_2 as i64,
        time: time.as_nanos(),
    };
}

// -----------------------------------------------------------------------------
// Line parsing regex
// -----------------------------------------------------------------------------
//lazy_static! {
//    static ref RE_LINE: Regex = Regex::new(
//        r"(?x)
//            (?P<lower>\d+) # lower requirement
//            -
//            (?P<upper>\d+) # upper requirement
//            \s
//            (?P<required>\w) # required character
//            :\s
//            (?P<password>\w+) # password to check
//            ",
//    )
//    .unwrap();
//}
// -----------------------------------------------------------------------------
