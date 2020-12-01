use std::fs::File;
use std::io::{BufRead, BufReader};

// -----------------------------------------------------------------------------
// Day 1
// -----------------------------------------------------------------------------
crate fn run() {
    println!("- Day 1:");

    // -------------------------------------------------------------------------
    // Part 1
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
    const YEAR: usize = 2020;
    let mut mask = [false; YEAR + 1];
    for value in &values {
        mask[*value] = true;
    }

    // Look for pair
    let index = mask
        .iter()
        .enumerate()
        .zip(mask.iter().rev())
        .find_map(|((i, a), b)| if a & b { Some(i) } else { None })
        .unwrap();
    let tuple = (index, YEAR - index);

    // Report
    println!("    Part 1:");
    println!("      Values : {}, {}", tuple.0, tuple.1);
    println!("      Product: {}", tuple.0 * tuple.1);

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let mut triple = (0, 0, 0);
    for value in &values {
        let chunk_size = YEAR - *value + 1;
        let chunk = mask.chunks(chunk_size).next().unwrap();
        let index = chunk
            .iter()
            .enumerate()
            .zip(chunk.iter().rev())
            .find_map(|((i, a), b)| if a & b { Some(i) } else { None });
        if index != None {
            triple = (*value, index.unwrap(), YEAR - *value - index.unwrap());
            break;
        }
    }

    // Report
    println!("    Part 2:");
    println!("      Values : {}, {}, {}", triple.0, triple.1, triple.2);
    println!("      Product: {}", triple.0 * triple.1 * triple.2);
}
