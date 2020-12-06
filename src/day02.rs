use crate::prelude::*;

// -----------------------------------------------------------------------------
// Password data struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct PasswordData {
    lower: usize,
    upper: usize,
    required: char,
    password: String,
}

impl std::str::FromStr for PasswordData {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(&['-', ' ', ':'][..]);
        let lower = line.next().unwrap().parse()?;
        let upper = line.next().unwrap().parse()?;
        let required = line.next().unwrap().chars().nth(0).unwrap();
        assert_eq!(line.next(), Some(""));
        let password = line.next().unwrap().to_string();
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
    (data.lower..=data.upper).contains(
        &data
            .password
            .as_bytes()
            .iter()
            .filter(|&c| *c == data.required as u8)
            .count(),
    )
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(data: &PasswordData) -> bool {
    let chars = data.password.as_bytes();
    let first = chars[data.lower - 1] == data.required as u8;
    let second = chars[data.upper - 1] == data.required as u8;
    first ^ second
}

// -----------------------------------------------------------------------------
// Combined
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct PasswordValidityData {
    part_1: bool,
    part_2: bool,
}

impl std::str::FromStr for PasswordValidityData {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(&['-', ' ', ':'][..]);
        let lower = line.next().unwrap().parse()?;
        let upper = line.next().unwrap().parse()?;
        let required = line.next().unwrap().chars().nth(0).unwrap();
        assert_eq!(line.next(), Some(""));
        let password = line.next().unwrap().to_string();
        // Part 1
        let chars = password.as_bytes();
        let part_1 =
            (lower..=upper).contains(&chars.iter().filter(|&c| *c == required as u8).count());
        // Part 2
        let first = chars[lower - 1] == required as u8;
        let second = chars[upper - 1] == required as u8;
        let part_2 = first ^ second;
        // Return
        Ok(Self { part_1, part_2 })
    }
}

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    if print_summary {
        output::print_day(2);
    }
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Data
    // -------------------------------------------------------------------------
    // Open file
    let buffer: String = std::fs::read_to_string("data/day02.txt").unwrap();

    // Read to object iterator
    let data: Vec<PasswordData> = buffer
        .lines()
        .map(|line| {
            line.parse::<PasswordData>()
                .expect("failed to parse password")
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
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let (combined_1, combined_2) = buffer
        .lines()
        .map(|line| {
            line.parse::<PasswordValidityData>()
                .expect("failed to parse password")
        })
        .fold((0, 0), |acc, data| {
            (acc.0 + data.part_1 as usize, acc.1 + data.part_2 as usize)
        });
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, count_1);
    assert_eq!(combined_2, count_2);

    // -------------------------------------------------------------------------
    // Report
    // -------------------------------------------------------------------------
    if print_summary {
        output::print_setup(time_setup);
        output::print_part(
            1,
            "Rule",
            "required number",
            "🔑 Valid",
            &format!("{}", count_1),
            time_part_1,
        );
        output::print_part(
            2,
            "Rule",
            "only one of two",
            "🔑 Valid",
            &format!("{}", count_2),
            time_part_2,
        );
        output::print_timing(time, time_part_1, time_part_2, time_combined);
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part_1: count_1 as i64,
        part_2: count_2 as i64,
        time: time_combined,
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
