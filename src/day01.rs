use std::fs::File;
use std::io::{BufRead, BufReader};

// -----------------------------------------------------------------------------
// Day 1
// -----------------------------------------------------------------------------
crate fn run() {
    println!("- Day 1");
    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Open file
    let path = "data/day01part01.txt";
    let input = File::open(path).unwrap();
    let buffer = BufReader::new(input);

    // Read to vector
    let mut values: Vec<usize> = Vec::new();
    for (_, line) in buffer.lines().enumerate() {
        values.push(line.unwrap().parse().unwrap());
    }

    // Mask array
    const YEAR: usize = 2020;
    let mut mask = [false; YEAR + 1];
    for value in values {
        mask[value] = true;
    }

    // Look for pair
    let index = mask
        .iter()
        .enumerate()
        .zip(mask.iter().rev())
        .find_map(|((i, a), b)| if a & b { Some(i) } else { None })
        .unwrap();

    // Report
    println!("  Part 1:");
    println!("    Product: {}", index * (2020 - index));

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
}
