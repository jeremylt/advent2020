use crate::prelude::*;

// -----------------------------------------------------------------------------
// Passport data struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct PassportData {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
    len: usize,
}

macro_rules! copy_five {
    ($e:expr) => {
        [$e, $e, $e, $e, $e]
    };
}

impl std::str::FromStr for PassportData {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.trim().split(&['\n', ' '][..]);
        let [mut byr, mut iyr, mut eyr]: [i32; 3] = [0; 3];
        let [mut hgt, mut hcl, mut ecl, mut pid, mut cid]: [String; 5] =
            copy_five!("invalid".to_string());
        let mut len: usize = 0;
        for field in line {
            len += 1;
            let mut entry = field.splitn(2, ':');
            let name = entry.next().unwrap();
            let data = entry.next().unwrap().to_string();
            match name {
                "byr" => byr = data.parse()?,
                "iyr" => iyr = data.parse()?,
                "eyr" => eyr = data.parse()?,
                "hgt" => hgt = data,
                "hcl" => hcl = data,
                "ecl" => ecl = data,
                "pid" => pid = data,
                "cid" => cid = data,
                _ => panic!("Unknown field"),
            }
        }
        Ok(Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
            len,
        })
    }
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(data: &PassportData) -> bool {
    (data.len == 8) || (data.len == 7 && data.cid == "invalid")
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(data: &PassportData) -> bool {
    // Number of fields
    (data.len >= 7)
        // Birth year
        && (1920..=2002).contains(&data.byr)
        // Issue year
        && (2010..=2020).contains(&data.iyr)
        // Expire year
        && (2020..=2030).contains(&data.eyr)
        // Eye color
        && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&data.ecl.as_str())
        // Hair color
        && (data.hcl.chars().nth(0).unwrap() == '#')
        && (data.hcl.len() == 7)
        && (data
            .hcl
            .chars()
            .skip(1)
            .all(|x| x.is_digit(16)))
        // Passport ID
        && (data.pid.len() == 9)
        && (data.pid.chars().all(char::is_numeric))
        // Height
        && ((data.hgt.chars().last().unwrap() == 'm'
            && (150..=193).contains(&data.hgt.replace("cm", "").parse::<i32>().unwrap()))
            || (data.hgt.chars().last().unwrap() == 'n'
                && (59..=76).contains(&data.hgt.replace("in", "").parse::<i32>().unwrap())))
}

// -----------------------------------------------------------------------------
// Day 2
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    if print_summary {
        output::print_day(4);
    }
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Data
    // -------------------------------------------------------------------------
    // Open file
    let buffer = std::fs::read_to_string("data/day04.txt").unwrap();

    // Read to object iterator
    let data: Vec<PassportData> = buffer
        .split("\n\n")
        .map(|line| {
            line.parse::<PassportData>()
                .expect("failed to parse passport")
        })
        .collect();

    // Timing
    let time_setup = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find matching passports
    let start_part_1 = Instant::now();
    let count_1 = data.iter().filter(|&d| part_1(d)).count();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find matching passports
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
            "cid optional",
            "📘 Valid",
            &format!("{}", count_1),
            time_part_1,
        );
        output::print_part(
            2,
            "Rule",
            "validated entries",
            "📘 Valid",
            &format!("{}", count_2),
            time_part_2,
        );
        output::print_timing(time, time_part_1, time_part_2);
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part_1: count_1 as i64,
        part_2: count_2 as i64,
        time: time.as_nanos(),
    };
}

// -----------------------------------------------------------------------------
