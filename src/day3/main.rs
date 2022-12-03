use std::collections::HashSet;
use std::fs;

const POSSIBLE_ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("AOC day 3..");
    let input: String = fs::read_to_string("./src/day3/input.txt").expect("File should exist");
    pt1(input);
}

fn pt1(input: String) -> () {
    // Start Part One
    println!("Part One");

    // Initialize sum of priorities
    let mut priority_sum: u32 = 0;
    for line in input.lines() {
        let mut line_priority_score: u32 = 0;

        // Split item types in two HashSets resembling equal compartments
        let item_count_per_compartment: usize = line.chars().count() / 2;
        let first_compartment_set: HashSet<_> =
            line[..item_count_per_compartment].chars().collect();
        let second_compartment_set: HashSet<_> =
            line[item_count_per_compartment..].chars().collect();

        // Intersect two HashSets to find duplicate items
        let duplicate_items = first_compartment_set.intersection(&second_compartment_set);

        // Loop through duplicate items and add priority score for that item type
        duplicate_items.for_each(|item| {
            let priority = POSSIBLE_ITEMS
                .find(item.clone())
                .expect("Item type should exist")
                + 1;
            line_priority_score += priority as u32;
        });

        // Add line priorities to total sum of priorities
        priority_sum += line_priority_score;
    }

    println!("The answer for Part One is: {}", priority_sum);
}
