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
        let [mut byr, mut iyr, mut eyr]: [i32; 3] = [0; 3];
        let [mut hgt, mut hcl, mut ecl, mut pid, mut cid]: [String; 5] =
            copy_five!("invalid".to_string());
        let mut len: usize = 0;
        for field in s.trim().split(&['\n', ' '][..]) {
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
    (data.len == 8) ^ (data.len == 7 && data.cid == "invalid")
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
// Birth year
fn byr_valid(byr: i32) -> bool {
    (1920..=2002).contains(&byr)
}

// Issue year
fn iyr_valid(iyr: i32) -> bool {
    (2010..=2020).contains(&iyr)
}

// Expire year
fn eyr_valid(eyr: i32) -> bool {
    (2020..=2030).contains(&eyr)
}

// Eye color
fn ecl_valid(ecl: &String) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
}

// Hair color
fn hcl_valid(hcl: &String) -> bool {
    (hcl.as_bytes()[0] == b'#') && (hcl.len() == 7) && (hcl.chars().skip(1).all(|x| x.is_digit(16)))
}

// Passport ID
fn pid_valid(pid: &String) -> bool {
    (pid.len() == 9) && (pid.chars().all(char::is_numeric))
}

// Passport ID
fn hgt_valid(hgt: &String) -> bool {
    (hgt.chars().last().unwrap() == 'm'
        && (150..=193).contains(&hgt.replace("cm", "").parse::<i32>().unwrap()))
        || (hgt.chars().last().unwrap() == 'n'
            && (59..=76).contains(&hgt.replace("in", "").parse::<i32>().unwrap()))
}

fn part_2(data: &PassportData) -> bool {
    (data.len >= 7)
        && byr_valid(data.byr)
        && iyr_valid(data.iyr)
        && eyr_valid(data.eyr)
        && ecl_valid(&data.ecl)
        && hcl_valid(&data.hcl)
        && pid_valid(&data.pid)
        && hgt_valid(&data.hgt)
}

// -----------------------------------------------------------------------------
// Combined
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct PassportValidityData {
    part_1: bool,
    part_2: bool,
}

impl std::str::FromStr for PassportValidityData {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = 0;
        let mut valid_fields = 0;
        let mut cid = false;
        for field in s.trim().split(&['\n', ' '][..]) {
            fields += 1;
            let mut entry = field.splitn(2, ':');
            let name = entry.next().unwrap();
            let data = entry.next().unwrap().to_string();
            match name {
                "byr" => valid_fields += byr_valid(data.parse().unwrap()) as usize,
                "iyr" => valid_fields += iyr_valid(data.parse().unwrap()) as usize,
                "eyr" => valid_fields += eyr_valid(data.parse().unwrap()) as usize,
                "hgt" => valid_fields += hgt_valid(&data) as usize,
                "hcl" => valid_fields += hcl_valid(&data) as usize,
                "ecl" => valid_fields += ecl_valid(&data) as usize,
                "pid" => valid_fields += pid_valid(&data) as usize,
                "cid" => cid = true,
                _ => panic!("Unknown field"),
            }
        }
        let part_1 = (fields == 8) || (fields == 7 && !cid);
        let part_2 = valid_fields == 7;
        Ok(Self { part_1, part_2 })
    }
}

// -----------------------------------------------------------------------------
// Day 4
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
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let (combined_1, combined_2) = buffer
        .split("\n\n")
        .map(|line| {
            line.parse::<PassportValidityData>()
                .expect("failed to parse passport")
        })
        .fold((0, 0), |acc, passport| {
            (
                acc.0 + passport.part_1 as usize,
                acc.1 + passport.part_2 as usize,
            )
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
