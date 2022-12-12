use std::collections::VecDeque;
use std::fs;

const ELEVATION: &str = "abcdefghijklmnopqrstuvwxyz";

// Position is a tuple containing location and height value
// Eg. (x, y, height)
type Position = (usize, usize, isize);

fn main() {
    println!("--- Day 12: Hill Climbing Algorithm ---");

    let input: String = fs::read_to_string("./src/day12/input.txt").expect("File should exist");

    let mut grid: Vec<Vec<Position>> = Vec::new();
    let mut start_position: Position = (0, 0, 0);
    let mut end_position: Position = (0, 0, 0);

    input.lines().enumerate().for_each(|(row_i, line)| {
        grid.push(Vec::new());
        line.chars().enumerate().for_each(|(column_i, character)| {
            let mut position: Position = (row_i, column_i, 0);
            if character == 'S' {
                start_position = position;
            } else if character == 'E' {
                position.2 = 25;
                end_position = position;
            } else {
                let character_height = ELEVATION.find(character).unwrap();
                position.2 = character_height as isize;
            }
            grid[row_i].push(position);
        });
    });

    let mut queue: VecDeque<(Position, Vec<Position>)> = VecDeque::new();
    let mut visited: Vec<Position> = Vec::new();
    let mut shortest_path: Vec<Position> = Vec::new();

    queue.push_back((start_position, Vec::new()));

    while let Some((position, path)) = queue.pop_front() {
        if position == end_position {
            shortest_path = path.to_owned();
            break;
        }

        // Define neighbours
        let mut neighbours: Vec<Position> = Vec::new();

        // Left
        if position.0 > 0 {
            neighbours.push(grid[position.0 - 1][position.1]);
        }

        // Right
        if position.0 < grid.len() - 1 {
            neighbours.push(grid[position.0 + 1][position.1]);
        }

        // Up
        if position.1 > 0 {
            neighbours.push(grid[position.0][position.1 - 1]);
        }

        // Down
        if position.1 < grid[0].len() - 1 {
            neighbours.push(grid[position.0][position.1 + 1]);
        }

        // Filter neighbours to unvisited and equal or one increment in height
        let unexplored_neighbours: Vec<Position> = neighbours
            .into_iter()
            .filter(|neighbour| neighbour.2 <= (position.2 + 1) && !visited.contains(neighbour))
            .collect::<Vec<Position>>();

        // Push unexplored neighbour to the back of the queue
        unexplored_neighbours.iter().for_each(|position| {
            let mut new_path = path.to_owned();
            new_path.push(*position);

            visited.push(*position);
            queue.push_back((*position, new_path));
        });
    }

    // Part One
    println!("Part One: {:?}", shortest_path.len());
}
