use std::fs;
use std::str::Chars;


fn main() {
    println!("--- Day 6: Tuning Trouble ---");
    let input: String = fs::read_to_string("./src/day6/input.txt").expect("File should exist");

    let characters: Chars = input.chars();
    
    let part1: String = find_marker(characters.clone(), 4);
    let part2: String = find_marker(characters.clone(), 14);

    println!("Part One: {}", part1);
    println!("Part Two: {}", part2);
}

fn find_marker(characters: Chars, distinct_amount: usize) -> String {
    let mut result: String = String::new();

    let mut set: Vec<char> = Vec::new();
    characters.enumerate().for_each(| (i, character) | {
        if i > distinct_amount - 1 {
            set.pop();
        };

        set.insert(0, character);
        
        let mut dedupped_set = set.clone();
        dedupped_set.sort();
        dedupped_set.dedup();
        
        if dedupped_set.len() == distinct_amount && result.is_empty() {
            result = (i + 1).to_string();
        }
    });

    result
}
