use crate::prelude::*;

// Constants
const NUMBER_DASHES: usize = 80;

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
// Output day information
// -----------------------------------------------------------------------------
pub(crate) fn print_day(day: usize) {
    println!("- {}", format!("Day {:02}", day).bold());
}

// -----------------------------------------------------------------------------
// Output setup summary
// -----------------------------------------------------------------------------
pub(crate) fn print_setup(time: std::time::Duration) {
    println!("    {}:", "Setup".red().bold());
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
        "Part 1".blue().bold()
    } else {
        "Part 2".green().bold()
    };
    println!("    {}:", part_string);
    println!("      {}: {}", property, property_value);
    println!("      {}: {}", output, output_value);
    println!("      Time: {:?}", time);
}

// -----------------------------------------------------------------------------
// Output timing summary
// -----------------------------------------------------------------------------
pub(crate) fn print_timing(
    time_total: std::time::Duration,
    time_part_1: std::time::Duration,
    time_part_2: std::time::Duration,
    time_combined: std::time::Duration,
) {
    println!("    {}:", "Timing".purple().bold());
    let part_1_percent = time_part_1.as_nanos() as f64 / time_total.as_nanos() as f64;
    let part_1_portion = std::cmp::max(1, (NUMBER_DASHES as f64 * part_1_percent) as usize);
    let part_2_percent = time_part_2.as_nanos() as f64 / time_total.as_nanos() as f64;
    let part_2_portion = std::cmp::max(1, (NUMBER_DASHES as f64 * part_2_percent) as usize);
    let setup_portion = NUMBER_DASHES - part_1_portion - part_2_portion;
    println!(
        "      Setup: {}",
        format!("{:02.1}%", 100.0 * (1.0 - part_1_percent - part_2_percent)).red(),
    );
    println!(
        "      Part 1: {}",
        format!("{:02.1}%", 100.0 * part_1_percent).blue(),
    );
    println!(
        "      Part 2: {}",
        format!("{:02.1}%", 100.0 * part_2_percent).green(),
    );
    println!("      Total: {:?}", time_total,);
    println!(
        "      Combined Time: {}",
        if time_total != time_combined {
            format!(
                "{:?} ({:2.1}%)",
                time_combined,
                time_combined.as_nanos() as f64 / time_total.as_nanos() as f64 * 100.0
            )
        } else {
            "N/A".to_string()
        }
    );
    println!(
        "{}{}{}",
        "-".repeat(setup_portion).red().bold(),
        "-".repeat(part_1_portion).blue().bold(),
        "-".repeat(part_2_portion).green().bold()
    );
}

// -----------------------------------------------------------------------------
// Output timing comparison
// -----------------------------------------------------------------------------
pub(crate) fn print_days_timing(times: &Vec<u128>) {
    println!("- {}", "Timing Comparison".bold());
    let total: u128 = times.iter().sum::<u128>();
    let longest: f64 = (*times.iter().max().unwrap()) as f64;
    for (i, &time) in times.iter().enumerate() {
        let part_length = std::cmp::max(
            1,
            ((NUMBER_DASHES - 2) as f64 * (time as f64 / longest)) as usize,
        );
        let dashes = "-".repeat(part_length);
        println!("  Dec {:02}: {:?}ms", i + 1, time as f64 / 1000000.0);
        println!(
            "  {}",
            if i % 2 == 0 {
                dashes.red().bold()
            } else {
                dashes.green().bold()
            }
        );
    }
    println!(
        "  {}: {:?}ms",
        "Total".purple().bold(),
        total as f64 / 1000000.0
    );
}

// -----------------------------------------------------------------------------
