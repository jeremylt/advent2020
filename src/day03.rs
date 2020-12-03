use crate::prelude::*;

// -----------------------------------------------------------------------------
// Count trees
// -----------------------------------------------------------------------------
fn count_trees(data: &Vec<String>, right: usize, down: usize, line_length: usize) -> usize {
    data.iter()
        .enumerate()
        .filter(|(i, line)| {
            (i % down == 0) && (line.as_bytes()[right * i / down % line_length] == b'#')
        })
        .count()
}

// -----------------------------------------------------------------------------
// Day 3
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    if print_summary {
        println!("- {}", "Day 03".bold());
    }
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Data
    // -------------------------------------------------------------------------
    // Open file
    let buffer = file::load_file("data/day03part01.txt");

    // Read to object iterator
    let data: Vec<String> = buffer.lines().map(|line| line.unwrap()).collect();

    // Timing
    let time_setup = start_all.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Find trees on path
    let start_part_1 = Instant::now();
    let line_length = data[0].chars().count();
    let count_1 = count_trees(&data, 3, 1, line_length);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Find product of trees on paths
    let start_part_2 = Instant::now();
    let slopes = [(1, 1), (5, 1), (7, 1), (1, 2)];
    let product_2 = slopes.iter().fold(count_1, |product, &(right, down)| {
        product * count_trees(&data, right, down, line_length)
    });
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
            "Path",
            "right 3, down 1",
            "Count",
            &format!("{}", count_1),
            time_part_1,
        );
        output::print_part(
            2,
            "Paths",
            "(right, down) -> (1, 1), (3, 1), (5, 1), (7, 1), (1, 2)",
            "Product",
            &format!("{}", product_2),
            time_part_2,
        );
        output::print_timing(time, time_part_1, time_part_2);
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part1: count_1 as i64,
        part2: product_2 as i64,
        time: time.as_nanos(),
    };
}

// -----------------------------------------------------------------------------
