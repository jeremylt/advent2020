use crate::prelude::*;

// -----------------------------------------------------------------------------
// Passport data struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct PassportData {
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
        ($e, $e, $e, $e, $e)
    };
}

impl std::str::FromStr for PassportData {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<&str> = s.trim().split(|c| " ".contains(c)).collect();
        let (mut byr, mut iyr, mut eyr): (i32, i32, i32) = (0, 0, 0);
        let (mut hgt, mut hcl, mut ecl, mut pid, mut cid): (
            String,
            String,
            String,
            String,
            String,
        ) = copy_five!("invalid".to_string());
        let len: usize = line.len();
        for field in line {
            let mut entry = field.split(|c| ":".contains(c));
            let name = entry.next().unwrap();
            let data = entry.next().unwrap().to_string();
            match name {
                "byr" => byr = data.parse().unwrap(),
                "iyr" => iyr = data.parse().unwrap(),
                "eyr" => eyr = data.parse().unwrap(),
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
fn is_between(value: i32, min: i32, max: i32) -> bool {
    (value >= min) && (value <= max)
}
fn part_2(data: &PassportData) -> bool {
    (data.len >= 7)
        && is_between(data.byr, 1920, 2002)
        && is_between(data.iyr, 2010, 2020)
        && is_between(data.eyr, 2020, 2030)
        && (data.hcl.chars().nth(0).unwrap() == '#')
        && (data.hcl.len() == 7)
        && (data
            .hcl
            .chars()
            .skip(1)
            .all(|x| x.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&x)))
        && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&data.ecl.as_str())
        && (data.pid.len() == 9)
        && (data.pid.parse::<i32>().is_ok())
        && ((data.hgt.chars().last().unwrap() == 'm'
            && is_between(data.hgt.replace("cm", "").parse::<i32>().unwrap(), 150, 193))
            || (data.hgt.chars().last().unwrap() == 'n'
                && is_between(data.hgt.replace("in", "").parse::<i32>().unwrap(), 59, 76)))
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
    let buffer = file::load_file("data/day04.txt");

    // Read to object iterator
    let mut data: Vec<PassportData> = Vec::new();
    let mut entry = "".to_string();
    for line in buffer.lines() {
        let unwrapped = line.unwrap();
        if unwrapped == "" {
            data.push(
                entry
                    .parse::<PassportData>()
                    .expect("Could not parse entry"),
            );
            entry = "".to_string();
        } else {
            entry = format!("{} {}", entry, unwrapped);
        }
    }
    data.push(
        entry
            .parse::<PassportData>()
            .expect("Could not parse entry"),
    );

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
