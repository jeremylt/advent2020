use crate::prelude::*;

const YEAR: usize = 2020;

// -----------------------------------------------------------------------------
// Find pair of flaged indices that sum to length of mask array
// -----------------------------------------------------------------------------
fn find_two(array: &[bool]) -> Option<i32> {
    let value = array
        .iter()
        .enumerate()
        .zip(array.iter().rev())
        .find_map(|((i, a), b)| if a & b { Some(i as i32) } else { None });
    value
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(mask: &[bool]) -> (i32, i32) {
    let index: i32;
    match find_two(&mask) {
        Some(value) => index = value,
        None => panic!("No pair found"),
    }
    (index, YEAR as i32 - index)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(values: &Vec<usize>, mask: &[bool]) -> (i32, i32, i32) {
    for value in values {
        let chunk_size = YEAR - *value + 1;
        let chunk = mask.chunks(chunk_size).next().unwrap();
        let index = find_two(&chunk);
        if index != None {
            let triple = (
                *value as i32,
                index.unwrap(),
                YEAR as i32 - *value as i32 - index.unwrap(),
            );
            return triple;
        }
    }
    panic!("No triple found");
}

// -----------------------------------------------------------------------------
// Day 1
// -----------------------------------------------------------------------------
pub(crate) fn run() -> Results {
    println!("- {}", "Day 01".bold());
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let path = "data/day01part01.txt";
    let input: File;
    match File::open(path) {
        Ok(file) => input = file,
        Err(_error) => panic!("Unable to open input file"),
    }
    let buffer = BufReader::new(input);

    // Read to vector
    let mut values: Vec<usize> = Vec::new();
    for line in buffer.lines() {
        values.push(line.unwrap().parse().unwrap());
    }

    // Mask array
    let mut mask = [false; YEAR + 1];
    for value in &values {
        mask[*value] = true;
    }

    // Time
    let time_setup = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Look for pair
    let start_part_1 = Instant::now();
    let tuple = part_1(&mask);
    let product_1 = tuple.0 * tuple.1;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let start_part_2 = Instant::now();
    let triple = part_2(&values, &mask);
    let product_2 = triple.0 * triple.1 * triple.2;
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
    println!("      Values   : {}, {}", tuple.0, tuple.1);
    println!("      Product  : {}", product_1);
    println!("      Time     : {:?}", time_part_1);
    // Part 2
    println!("    {}:", "Part 2".blue().bold());
    println!("      Values   : {}, {}, {}", triple.0, triple.1, triple.2);
    println!("      Product  : {}", product_2);
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
        part1: product_1,
        part2: product_2,
        time: time.as_millis(),
    };
}

// -----------------------------------------------------------------------------
