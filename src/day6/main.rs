use std::fs;
use std::str::Chars;


fn main() {
    println!("--- Day 5: Supply Stacks ---");
    let input: String = fs::read_to_string("./src/day6/input.txt").expect("File should exist");

    let characters: Chars = input.chars();
    
    let part1: String = find_marker(characters.clone(), 4);
    let part2: String = find_marker(characters.clone(), 14);

    println!("Part One: {}", part1);
    println!("Part Two: {}", part2);
}

fn find_marker(characters: Chars, distinct_marker: usize) -> String {
    let mut result: String = String::new();

    let mut curr_set: Vec<char> = Vec::new();
    characters.enumerate().for_each(| (i, character) | {
        if i > distinct_marker - 1 {
            curr_set.pop();
        };

        curr_set.insert(0, character);
        
        let mut dedupped = curr_set.clone();
        dedupped.sort();
        dedupped.dedup();
        
        if dedupped.len() == distinct_marker && result.is_empty() {
            println!("Four different characters occurred at {}", i + 1);
            result = (i + 1).to_string();
        }
    });

    result
}
