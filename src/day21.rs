//! Day 21:
//! No particular tricks in this one.

use crate::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

// Constants
const CAPACITY: usize = 38;

// -----------------------------------------------------------------------------
// Food
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl std::str::FromStr for Food {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.splitn(2, " (contains ");
        let ingredients = line
            .next()
            .unwrap()
            .split(" ")
            .map(|item| item.to_string())
            .collect();
        let allergens = line
            .next()
            .unwrap()
            .splitn(2, ")")
            .next()
            .unwrap()
            .split(", ")
            .map(|allergen| allergen.to_string())
            .collect();
        Ok(Self {
            ingredients,
            allergens,
        })
    }
}

// -----------------------------------------------------------------------------
// Run
// -----------------------------------------------------------------------------
pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let buffer: String = std::fs::read_to_string("data/day21.txt").unwrap();

    // Read to vector
    let data: Vec<Food> = buffer
        .lines()
        .map(|line| line.parse::<Food>().expect("failed to parse food"))
        .collect();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // Count safe ingredients
    let start_part_1 = Instant::now();
    let mut allergen_ingredients_map =
        FxHashMap::<String, Vec<String>>::with_capacity_and_hasher(CAPACITY, Default::default());
    // Find possible allergens
    data.iter().for_each(|food| {
        food.allergens.iter().for_each(|allergen| {
            if allergen_ingredients_map.contains_key(allergen) {
                let ingredients = allergen_ingredients_map.get_mut(allergen).unwrap();
                if ingredients.len() != 1 {
                    ingredients.retain(|ingredient| food.ingredients.contains(ingredient));
                }
            } else {
                allergen_ingredients_map.insert(allergen.clone(), food.ingredients.clone());
            }
        })
    });

    // Set of ingredients with allergens
    let mut allergen_ingredients_set =
        FxHashSet::<String>::with_capacity_and_hasher(CAPACITY, Default::default());
    allergen_ingredients_map
        .iter()
        .for_each(|(_, ingredients)| {
            ingredients.iter().for_each(|ingredient| {
                allergen_ingredients_set.insert(ingredient.clone());
            })
        });
    // Count safe ingredients
    let count_1: usize = data
        .iter()
        .map(|food| {
            food.ingredients
                .iter()
                .filter(|&ingredient| !allergen_ingredients_set.contains(ingredient))
                .count()
        })
        .sum();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // List all allergen ingredients
    let start_part_2 = Instant::now();
    let number_allergens = allergen_ingredients_map.len();
    let mut found_allergens: Vec<(String, String)> = Vec::with_capacity(number_allergens);
    let mut all_found = 0;
    while all_found != number_allergens {
        let current = allergen_ingredients_map
            .iter()
            .find_map(|(allergen, ingredients)| {
                if ingredients.len() == 1 {
                    Some((allergen.clone(), ingredients[0].clone()))
                } else {
                    None
                }
            })
            .unwrap();
        allergen_ingredients_map.remove(&current.0);
        allergen_ingredients_map
            .iter_mut()
            .for_each(|(_, ingredients)| {
                ingredients.retain(|ingredient| *ingredient != current.1);
            });
        found_allergens.append(&mut vec![current]);
        all_found += 1;
    }
    found_allergens.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", found_allergens) // Uncomment to print answer
    let ordered_2 = all_found;
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        count_1 as i64,
        ordered_2 as i64,
        Timing::new(
            time_setup,
            time_part_1,
            time_part_2,
            std::time::Duration::new(0, 0),
        ),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(21, "Allergen Assessment");
    output::print_part(1, "🛒 Safe", &format!("{}", results.part_1));
    output::print_part(2, "🛒 Product", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
