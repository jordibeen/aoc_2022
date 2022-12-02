use std::fs;

#[derive(PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum ExpectedResult {
    Draw,
    Win,
    Loss,
}

fn main() {
    println!("AOC day 2..");

    // Read input.txt
    let input: String = fs::read_to_string("./src/day2/input.txt").expect("File should exist");

    // Start Part One
    println!("Part One");

    // Initialize total score
    let mut total_score_part_one: u32 = 0;
    for line in input.lines() {
        let picks: Vec<&str> = line.split(" ").collect();
        let opponents_pick = get_shape(picks[0]);
        let response_pick = get_shape(picks[1]);

        // Add line score to total score
        total_score_part_one += calculate_line_score(opponents_pick, response_pick);
    }

    println!("The answer for Part One is: {}", total_score_part_one);
    // End Part One

    // Start Part Two
    println!("Part Two");
    let mut total_score_part_two: u32 = 0;
    for line in input.lines() {
        let picks: Vec<&str> = line.split(" ").collect();
        let opponents_pick = get_shape(picks[0]);
        let expected_result = get_expected_result(picks[1]);
        let response_pick = get_shape_for_expected_result(&opponents_pick, expected_result);

        // Add line score to total score
        total_score_part_two += calculate_line_score(opponents_pick, response_pick);
    }

    println!("The answer for Part Two is: {}", total_score_part_two);
    // End Part Two
}

fn calculate_line_score(opponents_pick: Shape, response_pick: Shape) -> u32 {
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

fn get_expected_result(pick: &str) -> ExpectedResult {
    match pick {
        "X" => ExpectedResult::Loss,
        "Y" => ExpectedResult::Draw,
        "Z" => ExpectedResult::Win,
        _ => panic!("Invalid pick"),
    }
}

fn get_shape_for_expected_result(opponent_pick: &Shape, expected_result: ExpectedResult) -> Shape {
    match opponent_pick {
        Shape::Rock => match expected_result {
            ExpectedResult::Win => Shape::Paper,
            ExpectedResult::Draw => Shape::Rock,
            ExpectedResult::Loss => Shape::Scissors,
        },
        Shape::Paper => match expected_result {
            ExpectedResult::Win => Shape::Scissors,
            ExpectedResult::Draw => Shape::Paper,
            ExpectedResult::Loss => Shape::Rock,
        },
        Shape::Scissors => match expected_result {
            ExpectedResult::Win => Shape::Rock,
            ExpectedResult::Draw => Shape::Scissors,
            ExpectedResult::Loss => Shape::Paper,
        },
    }
}
