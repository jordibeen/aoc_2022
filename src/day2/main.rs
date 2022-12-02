use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    println!("AOC day 2..");

    // Read input.txt
    let input: String = fs::read_to_string("./src/day2/input.txt").expect("File should exist");

    // Initialize total score
    let mut total_score: u32 = 0;
    for line in input.lines() {
        // Add line score to total score
        total_score += calculate_line_score(line);
    }

    println!("Total score is: {}", total_score);
}

fn calculate_line_score(line: &str) -> u32 {
    let picks: Vec<&str> = line.split(" ").collect();
    let opponents_pick = get_shape(picks[0]);
    let response_pick = get_shape(picks[1]);

    // Initialize empty score
    let mut score: u32 = 0;

    // Add default response pick points
    match response_pick {
        Shape::Rock => score += 1,
        Shape::Paper => score += 2,
        Shape::Scissors => score += 3,
    };

    // Draw
    if opponents_pick == response_pick {
        score += 3;
    };

    // Paper beats Rock
    if opponents_pick == Shape::Rock && response_pick == Shape::Paper {
        score += 6;
    };

    // Scissors beats Paper
    if opponents_pick == Shape::Paper && response_pick == Shape::Scissors {
        score += 6;
    };

    // Rock beats Scissors
    if opponents_pick == Shape::Scissors && response_pick == Shape::Rock {
        score += 6;
    };

    score
}

fn get_shape(pick: &str) -> Shape {
    match pick {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Invalid pick"),
    }
}
