use serde_json::Value;
use std::fs;

fn main() {
    println!("--- Day 13: Distress Signal ---");

    let input: String = fs::read_to_string("./src/day13/input.txt").expect("File should exist");

    let mut correct_indices: Vec<usize> = Vec::new();
    input
        .split("\n\n")
        .into_iter()
        .enumerate()
        .for_each(|(i, pair_str)| {
            let pair: Vec<&str> = pair_str.split_whitespace().collect();
            let left_pair: Vec<Value> = serde_json::from_str(pair[0]).unwrap();
            let right_pair: Vec<Value> = serde_json::from_str(pair[1]).unwrap();

            if let Some(in_order) = check_in_order(&left_pair, &right_pair) {
                if in_order {
                    correct_indices.push(i + 1);
                }
            }
        });

    println!("Part One: {:?}", correct_indices.iter().sum::<usize>());
}

fn check_in_order(left_pair: &Vec<Value>, right_pair: &Vec<Value>) -> Option<bool> {
    // Loop through left items while keeping track of its index
    for i in 0..left_pair.len() {
        // !! Incorrect, right pair is smaller than the left pair
        if right_pair.get(i).is_none() {
            return Some(false);
        }

        // Parsing serde values..
        let left_value = &left_pair[i];
        let right_value = &right_pair[i];

        let left_number = left_value.as_i64();
        let right_number = right_value.as_i64();
        let left_array = left_value.as_array();
        let right_array = right_value.as_array();

        // Both pairs are numbers
        if let (Some(left_number), Some(right_number)) = (left_number, right_number) {
            // !! Correct, left number is smaller than right number
            if left_number < right_number {
                return Some(true);
            // !! Incorrect, left number is bigger than right number
            } else if left_number > right_number {
                return Some(false);
            }
        // Both pairs are lists, re-run recursively
        } else if let (Some(left_array), Some(right_array)) = (left_array, right_array) {
            if let Some(in_order) = check_in_order(left_array, right_array) {
                return Some(in_order);
            }
        // One of both pairs is a list, convert that one to a list and re-run recursively
        } else {
            let mut left_list = &vec![serde_json::to_value(left_number).unwrap()];
            let mut right_list = &vec![serde_json::to_value(right_number).unwrap()];
            if let Some(left_array) = left_array {
                left_list = left_array;
            }
            if let Some(right_array) = right_array {
                right_list = right_array;
            }
            if let Some(in_order) = check_in_order(left_list, right_list) {
                return Some(in_order);
            }
        }

        // !! Correct, it's the last iteration and left pair is smaller than right pair
        if i + 1 == (left_pair.len()) && right_pair.len() > i + 1 {
            return Some(true);
        }
    }

    // !! Correct, left pair is smaller (empty) than the right pair
    if left_pair.is_empty() && right_pair.len() > 0 {
        return Some(true);
    }

    // No confirmation, continue
    None
}
