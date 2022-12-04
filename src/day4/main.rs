use std::collections::HashSet;
use std::fs;

fn main() {
    println!("--- Day 4: Camp Cleanup ---");
    let input: String = fs::read_to_string("./src/day4/input.txt").expect("File should exist");
    pt1(&input);
    pt2(&input);
}

fn pt1(input: &String) -> () {
    // Start Part One
    println!("Part One");

    let mut count: u32 = 0;
    input.lines().for_each(|line| {
        // Save elf pairs to a Vec of HashSets containing their assigned ID numbers
        // Ex. [{2, 3, 4}, {5, 6, 7}]
        let mut pair_set: Vec<HashSet<u32>> = vec![];
        line.split(",").for_each(|elf_range| {
            let range: Vec<u32> = elf_range
                .split("-")
                .map(|val| val.parse::<u32>().expect("Range should be number"))
                .collect();

            let elf_set: HashSet<u32> = (range[0]..range[1] + 1).collect();
            pair_set.push(elf_set);
        });

        // If the first set of ID numbers fully contains the second set, or the second set of
        // ID numbers fully contains the first set, this line is counted a reconsideration pair
        if &pair_set[0].is_subset(&pair_set[1]) | &pair_set[1].is_subset(&pair_set[0]) {
            count += 1
        };
    });
    println!("The answer for Part One is: {}", count);
}

fn pt2(input: &String) -> () {
    // Start Part Two
    println!("Part Two");

    let mut count: u32 = 0;
    input.lines().for_each(|line| {
        // Save elf pairs to a Vec of HashSets containing their assigned ID numbers
        // Ex. [{2, 3, 4}, {5, 6, 7}]
        let mut pair_set: Vec<HashSet<u32>> = vec![];
        line.split(",").for_each(|elf_range| {
            let range: Vec<u32> = elf_range
                .split("-")
                .map(|val| val.parse::<u32>().expect("Range should be number"))
                .collect();

            let elf_set: HashSet<u32> = (range[0]..range[1] + 1).collect();
            pair_set.push(elf_set);
        });

        // If the first set of ID numbers intersects with the second set, this
        // line is counted an overlapping pair
        let intersection: HashSet<_> = pair_set[0].intersection(&pair_set[1]).collect();
        if intersection.len() > 0 {
            count += 1
        }
    });

    println!("The answer for Part Two is: {}", count);
}
