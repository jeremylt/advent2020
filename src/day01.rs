use std::fs::File;
use std::io::{BufRead, BufReader};

const YEAR: usize = 2020;

// -----------------------------------------------------------------------------
// Find pair of flaged indices that sum to length of mask array
// -----------------------------------------------------------------------------
fn find_two(array: &[bool]) -> Option<usize> {
    let value = array
        .iter()
        .enumerate()
        .zip(array.iter().rev())
        .find_map(|((i, a), b)| if a & b { Some(i) } else { None });
    value
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
fn part_1(mask: &[bool]) -> (usize, usize) {
    let index: usize;
    match find_two(&mask) {
        Some(value) => index = value,
        None => panic!("No pair found"),
    }
    (index, YEAR - index)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(values: &Vec<usize>, mask: &[bool]) -> (usize, usize, usize) {
    for value in values {
        let chunk_size = YEAR - *value + 1;
        let chunk = mask.chunks(chunk_size).next().unwrap();
        let index = find_two(&chunk);
        if index != None {
            let triple = (*value, index.unwrap(), YEAR - *value - index.unwrap());
            return triple;
        }
    }
    panic!("No triple found");
}

// -----------------------------------------------------------------------------
// Day 1
// -----------------------------------------------------------------------------
crate fn run() -> (usize, usize) {
    println!("- Day 1:");

    // -------------------------------------------------------------------------
    // Data
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
    for (_, line) in buffer.lines().enumerate() {
        values.push(line.unwrap().parse().unwrap());
    }

    // Mask array
    let mut mask = [false; YEAR + 1];
    for value in &values {
        mask[*value] = true;
    }

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Look for pair
    let tuple = part_1(&mask);
    let product_1 = tuple.0 * tuple.1;

    // Report
    println!("    Part 1:");
    println!("      Values : {}, {}", tuple.0, tuple.1);
    println!("      Product: {}", product_1);

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let triple = part_2(&values, &mask);
    let product_2 = triple.0 * triple.1 * triple.2;

    // Report
    println!("    Part 2:");
    println!("      Values : {}, {}, {}", triple.0, triple.1, triple.2);
    println!("      Product: {}", product_2);

    // Return
    return (product_1, product_2);
}

// -----------------------------------------------------------------------------
