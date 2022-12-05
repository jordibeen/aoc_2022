use regex::Regex;
use std::collections::VecDeque;
use std::fs;

fn main() {
    println!("--- Day 5: Supply Stacks ---");
    let input: String = fs::read_to_string("./src/day5/input.txt").expect("File should exist");

    // Split input into stacks of crates and move assignments
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut crates: Vec<&str> = split[0].split("\n").collect();
    let assignments: Vec<&str> = split[1].split("\n").collect();

    // Pop crate amount line from input crates and save the last crate number
    let max_crate_number = crates
        .pop()
        .expect("Should be a crate amount line")
        .chars()
        .last()
        .expect("Should have a last letter")
        .to_digit(10)
        .expect("Should be a digit");

    // Initialize Vector of empty VecDeques according to the amount of crate stacks
    // Ex. if max_crate_number is 4 = [[], [], [], []]
    let mut stacks: Vec<VecDeque<&str>> = Vec::new();
    for _ in 0..max_crate_number {
        stacks.push(VecDeque::new());
    }

    // Convert crates from input lines to vertical stacks of crates
    crates.iter().for_each(|crate_line| {
        crate_line.split("").enumerate().for_each(|(i, val)| {
            if let Some(letter) = Regex::new(r"\w").unwrap().find(val) {
                let stack_number: usize = (i - 2 % 4) / 4;
                stacks[stack_number].push_back(letter.as_str());
            }
        });
    });

    // Perform assignments
    let part1: String = perform_assignments(stacks.clone(), assignments.clone(), false);
    // Perform assignments using cratemover9001
    let part2: String = perform_assignments(stacks.clone(), assignments.clone(), true);

    // Print results
    println!("Part One: {}", part1);
    println!("Part Two: {}", part2);
}

fn perform_assignments(
    mut stacks: Vec<VecDeque<&str>>,
    assignments: Vec<&str>,
    cratemover9001: bool,
) -> String {
    assignments.iter().for_each(|assignment_line| {
        // Save Vector of digits from assignment line using regex
        // Ex. "move 8 from 2 to 4" -> [8, 2, 4]
        let digits: Vec<usize> = Regex::new(r"\d{1,3}")
            .unwrap()
            .find_iter(assignment_line)
            .map(|x| x.as_str().parse::<usize>().expect("Should be a number"))
            .collect();

        let move_amount = digits[0];
        let stack_from = digits[1];
        let stack_to = digits[2];

        // Start looping with an index from for how many moves are in this assignment
        // Ex. "move 8 from 2 to 4" -> 0..7
        for i in 0..move_amount {
            // Pop the first crate from the "from" stack, this is the crate to move
            let crate_to_move = &stacks[stack_from - 1]
                .pop_front()
                .expect("Should have an item");

            // Part 1 or Part 2 statement
            if !cratemover9001 {
                // Push the crate to move to the "to" stack
                let _ = &stacks[stack_to - 1].push_front(crate_to_move);
            } else {
                // Insert the crate to move a the correct index
                let _ = &stacks[stack_to - 1].insert(i, crate_to_move);
            }
        }
    });

    // Save first element of each stack to a result string
    let mut result: String = String::new();
    for stack in &stacks {
        result.push_str(stack[0]);
    }

    result
}
