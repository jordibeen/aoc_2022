use std::collections::HashSet;
use std::fs;

type Position = (isize, isize);

fn main() {
    println!("--- Day 9: Rope Bridge ---");
    let input: String = fs::read_to_string("./src/day9/input.txt").expect("File should exist");

    let mut head_position: Position = (0, 0);
    let mut tail_position: Position = (0, 0);
    let mut visited_positions: HashSet<Position> = HashSet::new();

    input.lines().for_each(|line| {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let direction = split[0];
        let steps = split[1].parse::<i32>().expect("Should be a number");

        for _ in 0..steps {
            match direction {
                "R" => head_position.0 += 1,
                "L" => head_position.0 -= 1,
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                _ => {}
            }

            move_tail_to_head(&mut tail_position, &mut head_position);
            visited_positions.insert(tail_position);
        }
    });

    println!("Part 1: {}", visited_positions.len());
}

fn move_tail_to_head(tail_position: &mut Position, head_position: &mut Position) {
    // Get X-axis difference between head and tail positions
    let x_diff: isize = head_position.0 - tail_position.0;
    // Get Y-axis difference between head and tail positions
    let y_diff: isize = head_position.1 - tail_position.1;

    // X difference bigger or less than 1 indicates tail needs to move on Y
    if x_diff > 1 || x_diff < -1 {
        // X difference bigger than 1, tail should move towards the right
        if x_diff > 1 {
            tail_position.0 += 1;
        // X difference less than -1, tail should move towards the left
        } else if x_diff < -1 {
            tail_position.0 -= 1;
        }

        // Next to a Y-axis difference, there's also a X-axis difference
        // of 1 or -1, tail should move diagonally
        if y_diff == 1 || y_diff == -1 {
            tail_position.1 += y_diff;
        }
    }

    // Y difference bigger or less than 1 indicates tail needs to move on Y
    if y_diff > 1 || y_diff < -1 {
        // Y difference bigger than 1, tail should move towards the right
        if y_diff > 1 {
            tail_position.1 += 1;
        // Y difference less than -1, tail should move towards the left
        } else if y_diff < -1 {
            tail_position.1 -= 1;
        }

        // Next to a X-axis difference, there's also a Y-axis difference
        // of 1 or -1, tail should move diagonally
        if x_diff == 1 || x_diff == -1 {
            tail_position.0 += x_diff;
        }
    }
}
