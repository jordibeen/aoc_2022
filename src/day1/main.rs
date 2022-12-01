use std::fs;

fn main() {
    println!("AOC day 1..");
    // Init..
    let mut highest_calorie_count: u32 = 0;

    // Read input.txt
    let input: String = fs::read_to_string("./src/day1/input.txt").expect("File should exist");

    // Loop through input lines and keep track of calorie count per elf
    let mut calorie_count: u32 = 0;
    for line in input.lines() {
        // An empty line marks the end of an elf
        if line.is_empty() {
            // If the calorie count for this elf exceeds the highest calorie count
            // so far, save it as the new highest count
            if calorie_count > highest_calorie_count {
                highest_calorie_count = calorie_count;
            }

            // Reset calorie counter back to 0
            calorie_count = 0;
            continue;
        }

        // Add calories to calorie count of the current elf
        calorie_count += line.parse::<u32>().unwrap();
    }

    println!("The highest calorie count is: {}", highest_calorie_count);
}
