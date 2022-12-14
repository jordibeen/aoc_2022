use std::collections::HashSet;
use std::fs;

type Position = (isize, isize);

fn main() {
    println!("--- Day 9: Rope Bridge ---");
    let input: String = fs::read_to_string("./src/day9/input.txt").expect("File should exist");

    let mut head_position: Position = (0, 0);
    let mut tail_position: Position = (0, 0);
    let mut tail_visited_positions: HashSet<Position> = HashSet::new();

    let mut knot_positions: Vec<Position> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut last_knot_visited_positions: HashSet<Position> = HashSet::new();

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

            // Part One
            // Move rope with just a head and tail
            follow(&mut tail_position, &head_position);
            // Keep track of positions the tail visited
            tail_visited_positions.insert(tail_position);

            // Part Two
            // Move rope with 9 knots
            let mut previous_position = head_position;
            knot_positions.iter_mut().for_each(|knot| {
                follow(knot, &previous_position);
                previous_position = knot.clone();
            });
            // Keep track of positions the last knot visited
            last_knot_visited_positions.insert(
                knot_positions
                    .last()
                    .expect("Should have a last position")
                    .clone(),
            );
        }
    });

    println!("Part 1: {}", tail_visited_positions.len());
    println!("Part 2: {}", last_knot_visited_positions.len());
}

fn follow(from_pos: &mut Position, to_pos: &Position) {
    // Get X-axis difference between to and from positions
    let x_diff: isize = to_pos.0 - from_pos.0;
    // Get Y-axis difference between to and from positions
    let y_diff: isize = to_pos.1 - from_pos.1;

    // X difference bigger or less than 1 indicates from_pos needs to move on Y
    if x_diff > 1 || x_diff < -1 {
        // X difference bigger than 1, from_pos should move towards the right
        if x_diff > 1 {
            from_pos.0 += 1;
        // X difference less than -1, from_pos should move towards the left
        } else if x_diff < -1 {
            from_pos.0 -= 1;
        }

        // Next to a Y-axis difference, there's also a X-axis difference
        // of 1 or -1, from_pos should move diagonally
        if y_diff == 1 || y_diff == -1 {
            from_pos.1 += y_diff;
        }
    }

    // Y difference bigger or less than 1 indicates from_pos needs to move on Y
    if y_diff > 1 || y_diff < -1 {
        // Y difference bigger than 1, from_pos should move towards the right
        if y_diff > 1 {
            from_pos.1 += 1;
        // Y difference less than -1, from_pos should move towards the left
        } else if y_diff < -1 {
            from_pos.1 -= 1;
        }

        // Next to a X-axis difference, there's also a Y-axis difference
        // of 1 or -1, from_pos should move diagonally
        if x_diff == 1 || x_diff == -1 {
            from_pos.0 += x_diff;
        }
    }
}
