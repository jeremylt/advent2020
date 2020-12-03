use crate::prelude::*;

// -----------------------------------------------------------------------------
// Print header
// -----------------------------------------------------------------------------
pub(crate) fn print_header() {
    println!("{}", "-".repeat(NUMBER_DASHES).green().bold());
    println!(
        "{} {} {}",
        "-".repeat(NUMBER_DASHES / 2 - 10).red().bold(),
        "Advent of Code 2020".bold(),
        "-".repeat(NUMBER_DASHES / 2 - 11).red().bold()
    );
    println!("{}", "-".repeat(NUMBER_DASHES).green().bold());
}

// -----------------------------------------------------------------------------
// Output setup summary
// -----------------------------------------------------------------------------
pub(crate) fn print_setup(time: std::time::Duration) {
    println!("    {}:", "Setup".cyan().bold());
    println!("      Time: {:?}", time);
}

// -----------------------------------------------------------------------------
// Output part summary
// -----------------------------------------------------------------------------
pub(crate) fn print_part(
    part: usize,
    property: &str,
    property_value: &str,
    output: &str,
    output_value: &str,
    time: std::time::Duration,
) {
    let part_string = if part == 1 {
        "Part 1".yellow().bold()
    } else {
        "Part 2".blue().bold()
    };
    println!("    {}:", part_string);
    println!("      {}: {}", property, property_value);
    println!("      {}: {}", output, output_value);
    println!("      Time: {:?}", time);
}

// -----------------------------------------------------------------------------
// Output part summary
// -----------------------------------------------------------------------------
pub(crate) fn print_timing(
    time_total: std::time::Duration,
    time_part_1: std::time::Duration,
    time_part_2: std::time::Duration,
) {
    println!("    {} : {:?}", "Total Time".purple().bold(), time_total);
    let part_1_portion = std::cmp::max(
        1,
        (NUMBER_DASHES as f64 * (time_part_1.as_nanos() as f64 / time_total.as_nanos() as f64))
            as usize,
    );
    let part_2_portion = std::cmp::max(
        1,
        (NUMBER_DASHES as f64 * (time_part_2.as_nanos() as f64 / time_total.as_nanos() as f64))
            as usize,
    );
    let setup_portion = NUMBER_DASHES - part_1_portion - part_2_portion;
    println!(
        "{}{}{}",
        "-".repeat(setup_portion).cyan().bold(),
        "-".repeat(part_1_portion).yellow().bold(),
        "-".repeat(part_2_portion).blue().bold()
    );
}

// -----------------------------------------------------------------------------
