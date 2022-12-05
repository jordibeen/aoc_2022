use regex::Regex;
use std::collections::VecDeque;
use std::fs;

fn main() {
    println!("--- Day 5: Supply Stacks ---");
    let input: String = fs::read_to_string("./src/day5/input.txt").expect("File should exist");

    let split: Vec<&str> = input.split("\n\n").collect();

    let mut crates: Vec<&str> = split[0].split("\n").collect();
    let assignments: Vec<&str> = split[1].split("\n").collect();

    let crate_amount_line = crates.pop().expect("Should be a crate amount line");
    let crate_amount: Vec<&str> = crate_amount_line.split(" ").collect();
    let crate_max_number: u32 = crate_amount[crate_amount.len() - 1]
        .parse()
        .expect("Should be a number");

    let mut stacks: Vec<VecDeque<&str>> = Vec::new();
    for _ in 0..crate_max_number {
        stacks.push(VecDeque::new());
    }

    for crate_line in crates {
        crate_line.split("").enumerate().for_each(|(i, val)| {
            if let Some(letter) = Regex::new(r"\w").unwrap().find(val) {
                let stack_number: usize = (i - 2 % 4) / 4;
                stacks[stack_number].push_back(letter.as_str());
            }
        });
    }

    println!(
        "Part One result: {}",
        perform_assignments(stacks.clone(), assignments.clone(), false),
    );
    println!(
        "Part Two result: {}",
        perform_assignments(stacks.clone(), assignments.clone(), true),
    );
}

fn perform_assignments(
    mut stacks: Vec<VecDeque<&str>>,
    assignments: Vec<&str>,
    cratemover9001: bool,
) -> String {
    for assignment in assignments {
        let digits: Vec<usize> = Regex::new(r"\d{1,3}")
            .unwrap()
            .find_iter(assignment)
            .map(|x| x.as_str().parse::<usize>().expect("Should be a number"))
            .collect();

        for i in 0..digits[0] {
            let crate_to_move = &stacks[digits[1] - 1]
                .pop_front()
                .expect("Should have an item");

            // Part 1
            if !cratemover9001 {
                let _ = &stacks[digits[2] - 1].push_front(crate_to_move);
            // Part 2
            } else {
                let _ = &stacks[digits[2] - 1].insert(i, crate_to_move);
            }
        }
    }

    let mut result: String = String::new();
    for stack in &stacks {
        result.push_str(stack[0]);
    }

    result
}
