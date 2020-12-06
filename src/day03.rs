use crate::prelude::*;

// -----------------------------------------------------------------------------
// Count trees
// -----------------------------------------------------------------------------
fn hit_tree(line: &String, i: &usize, right: usize, down: usize, line_length: usize) -> bool {
    (i % down == 0) && (line.as_bytes()[right * i / down % line_length] == b'#')
}

fn count_trees(data: &Vec<String>, right: usize, down: usize, line_length: usize) -> usize {
    data.iter()
        .enumerate()
        .filter(|(i, line)| hit_tree(*line, i, right, down, line_length))
        .count()
}

// -----------------------------------------------------------------------------
// Day 3
// -----------------------------------------------------------------------------
pub(crate) fn run(print_summary: bool) -> Results {
    if print_summary {
        output::print_day(3);
    }
    let start_all = Instant::now();

    // -------------------------------------------------------------------------
    // Data
    // -------------------------------------------------------------------------
    // Open file
    let buffer: String = std::fs::read_to_string("data/day03.txt").unwrap();

    // Read to object iterator
    let data: Vec<String> = buffer.lines().map(|line| line.to_string()).collect();

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
    // Combined
    // -------------------------------------------------------------------------
    let start_combined = Instant::now();
    let slopes = [(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees_hit = [0; 5];
    buffer
        .lines()
        .map(|line| line.to_string())
        .enumerate()
        .for_each(|(i, line)| {
            trees_hit
                .iter_mut()
                .zip(&slopes)
                .for_each(|(curr, &(right, down))| {
                    *curr += hit_tree(&line, &i, right, down, line_length) as usize
                });
        });
    let combined_1 = trees_hit[0];
    let combined_2: usize = trees_hit.iter().product();
    let time_combined = start_combined.elapsed();
    assert_eq!(combined_1, count_1);
    assert_eq!(combined_2, product_2);

    // -------------------------------------------------------------------------
    // Report
    // -------------------------------------------------------------------------
    if print_summary {
        output::print_setup(time_setup);
        output::print_part(
            1,
            "Path",
            "(right, down) -> (3, 1)",
            "🌲 Count",
            &format!("{}", count_1),
            time_part_1,
        );
        output::print_part(
            2,
            "Paths",
            "(right, down) -> (1, 1), (3, 1), (5, 1), (7, 1), (1, 2)",
            "🌲 Product",
            &format!("{}", product_2),
            time_part_2,
        );
        output::print_timing(time, time_part_1, time_part_2, time_combined);
    }

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    return Results {
        part_1: count_1 as i64,
        part_2: product_2 as i64,
        time: time.as_nanos(),
    };
}

// -----------------------------------------------------------------------------
