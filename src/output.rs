//! Output:
//! This module collects some of my `println!` boilerplate between the days.

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
pub(crate) fn print_day(day: usize, name: &str) {
    println!("- {}", format!("Day {:02} --- {} ---", day, name).bold());
}

// -----------------------------------------------------------------------------
// Output part summary
// -----------------------------------------------------------------------------
pub(crate) fn print_part(part: usize, output: &str, output_value: &str) {
    let part_string = if part == 1 {
        "Part 1".red().bold()
    } else {
        "Part 2".green().bold()
    };
    println!("    {}:", part_string);
    println!("      {}: {}", output, output_value);
}

// -----------------------------------------------------------------------------
// Output timing summary
// -----------------------------------------------------------------------------
pub(crate) fn print_timing(times: &Timing) {
    println!("    {}:", "Timing".purple().bold());
    let times_total = times.setup + times.part_1 + times.part_2;
    let part_1_percent = times.part_1.as_nanos() as f64 / times_total.as_nanos() as f64;
    let mut part_1_portion = std::cmp::max(1, (NUMBER_DASHES as f64 * part_1_percent) as usize);
    let part_2_percent = times.part_2.as_nanos() as f64 / times_total.as_nanos() as f64;
    let mut part_2_portion = std::cmp::max(1, (NUMBER_DASHES as f64 * part_2_percent) as usize);
    let mut setup_portion = NUMBER_DASHES - part_1_portion - part_2_portion;
    if setup_portion == 0 {
        setup_portion = 1;
        if part_1_portion > part_2_portion {
            part_1_portion -= 1;
        } else {
            part_2_portion -= 1;
        }
    }
    println!(
        "      {}: {:?} ({:02.1}%)",
        "Setup".blue(),
        times.setup,
        100.0 * (1.0 - part_1_percent - part_2_percent),
    );
    println!(
        "      {}: {:?} ({:02.1}%)",
        "Part 1".red(),
        times.part_1,
        100.0 * part_1_percent,
    );
    println!(
        "      {}: {:?} ({:02.1}%)",
        "Part 2".green(),
        times.part_2,
        100.0 * part_2_percent,
    );
    println!("      Total: {:?}", times_total,);
    println!(
        "      Combined Time: {}",
        if times.combined.as_nanos() > 1 {
            format!(
                "{:?} ({:2.1}%)",
                times.combined,
                times.combined.as_nanos() as f64 / times_total.as_nanos() as f64 * 100.0
            )
        } else {
            "N/A".to_string()
        }
    );
    println!(
        "{}{}{}",
        "-".repeat(setup_portion).blue().bold(),
        "-".repeat(part_1_portion).red().bold(),
        "-".repeat(part_2_portion).green().bold()
    );
}

// -----------------------------------------------------------------------------
// Output timing comparison
// -----------------------------------------------------------------------------
pub(crate) fn print_days_timing(times: &Vec<std::time::Duration>, std_devs: &Vec<f64>) {
    println!("- {}", "Timing Comparison".bold());
    println!("    {}: {}", "Repetitions".purple().bold(), REPETITIONS);
    println!("    {}: Logarithmic", "Scale".purple().bold());
    println!("    {}", "-".repeat(NUMBER_DASHES - 4).blue().bold());
    let total: std::time::Duration = times.iter().sum();
    let longest = (times.iter().max().unwrap().as_nanos() as f64).log10();
    for (i, (&time, &std_dev)) in times.iter().zip(std_devs).enumerate() {
        let part_length = std::cmp::max(
            1,
            ((NUMBER_DASHES - 4) as f64 * ((time.as_nanos() as f64).log10() / longest)) as usize,
        );
        let dashes = "-".repeat(part_length);
        println!("    Dec {:02}: {:?} (Std Dev {:.3})", i + 1, time, std_dev);
        println!(
            "    {}",
            if i % 2 == 0 {
                dashes.red().bold()
            } else {
                dashes.green().bold()
            }
        );
    }
    println!("    {}: {:?}", "Total".purple().bold(), total);
}

// -----------------------------------------------------------------------------
