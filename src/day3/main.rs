use std::collections::HashSet;
use std::fs;

const POSSIBLE_ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    println!("AOC day 3..");
    let input: String = fs::read_to_string("./src/day3/input.txt").expect("File should exist");
    pt1(&input);
    pt2(&input);
}

fn pt1(input: &String) -> () {
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

fn pt2(input: &String) -> () {
    // Start Part Two
    println!("Part Two");

    // Initialize sum of priorities
    let mut priority_sum: u32 = 0;

    // Initialize empty intersection set
    let mut intersection_set: HashSet<_> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        // Save current line's characters to a HashSet
        let line_set: HashSet<_> = line.chars().collect();

        if intersection_set.is_empty() {
            // If the HashSet just got initialized, set the current line as HashSet
            intersection_set = line_set;
        } else {
            // Else we are on the second or third elf in a group, so we intersect this line
            // with the active intersection set to check what item exists in both sets,
            // and save that result as the new intersection set
            intersection_set = line_set.intersection(&intersection_set).cloned().collect();
        }

        // If the line number is divisble by 3, it marks the end of an Elf group
        if (i + 1) % 3 == 0 {
            // Add priority score for this group to the total sum of priorities
            for item in intersection_set {
                let priority = POSSIBLE_ITEMS
                    .find(item.clone())
                    .expect("Item type should exist")
                    + 1;
                priority_sum += priority as u32;
            }

            // Re-initialize empty intersection set for the start of a new Elf group
            intersection_set = HashSet::new();
        }
    }

    println!("The answer for Part Two is: {}", priority_sum);
}
