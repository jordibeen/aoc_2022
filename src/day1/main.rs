use std::fs;

fn main() {
    println!("AOC day 1..");

    // Start Part One
    println!("Part One");

    // Init..
    let mut calories: Vec<u32> = vec![];

    // Read input.txt
    let input: String = fs::read_to_string("./src/day1/input.txt").expect("File should exist");

    // Loop through input lines and keep track of calorie count per elf
    let mut calorie_count: u32 = 0;
    for line in input.lines() {
        // An empty line marks the end of an elf
        if line.is_empty() {
            // Push the calories for this elf to the calories Vec
            calories.push(calorie_count);

            // Reset calorie counter back to 0
            calorie_count = 0;
            continue;
        }

        // Add calories to calorie count of the current elf
        calorie_count += line.parse::<u32>().unwrap();
    }

    // Get the highest value of the calories vector
    let highest_calorie_count = calories.iter().max().expect("Should have a highest count");
    println!("The answer for Part One is: {}", highest_calorie_count);
    // End Part One

    // Start Part Two
    println!("Part Two");

    // Sort the calories vector in descending order
    calories.sort();
    calories.reverse();

    // Get the sum of the three highest calorie counts
    let mut three_highest_calorie_count: u32 = 0;
    for i in 0..3 {
        three_highest_calorie_count += calories[i];
    }

    println!(
        "The answer for Part Two is: {}",
        three_highest_calorie_count
    );
    // End Part Two
}
