use std::collections::HashMap;
use std::fs;

// Position is a tuple containing (x, y)-points
// Eg. (494, 7)
type Position = (usize, usize);

#[derive(Debug)]
enum Occupation {
    Air,
    Rock,
    Sand,
}

fn main() {
    println!("--- Day 14: Regolith Reservoir ---");

    // Keeping track of what occupies a certain position
    // Eg. ((494,5), Rock)
    let mut occupation: HashMap<Position, Occupation> = HashMap::new();

    // Keeping track of grid dimensions (x-min, x-max, y-min, y-max)
    // Eg. (494, 503, 0, 9)
    let mut dimensions: (usize, usize, usize, usize) = (0, 0, 0, 0);

    let input: String = fs::read_to_string("./src/day14/input.txt").expect("File should exist");

    // Parse
    input.lines().for_each(|line| {
        let points_str: Vec<&str> = line.split(" -> ").collect();

        let mut previous_point: Option<Position> = None;
        points_str.iter().for_each(|point_str| {
            let coords: Vec<&str> = point_str.split(",").collect();
            let x: usize = coords[0].parse::<usize>().unwrap();
            let y: usize = coords[1].parse::<usize>().unwrap();

            let mut points: Vec<Position> = Vec::from([(x, y)]);
            if let Some(prev) = previous_point {
                // Indicates a straight line on x-axis
                if prev.0 > x {
                    (x..prev.0).for_each(|i| {
                        points.push((i, y));
                    })
                } else if prev.0 < x {
                    (prev.0..x).for_each(|i| {
                        points.push((i, y));
                    })
                }
                // Indicates a straight line on y-axis
                if prev.1 > y {
                    (y..prev.1).for_each(|i| {
                        points.push((x, i));
                    })
                } else if prev.1 < y {
                    (prev.1..y).for_each(|i| {
                        points.push((x, i));
                    })
                }
            };

            points.iter().for_each(|point| {
                occupation.insert(point.to_owned(), Occupation::Rock);
            });

            previous_point = Some((x, y));

            // Save dimensions
            if x < dimensions.0 || dimensions.0 == 0 {
                dimensions.0 = x;
            }
            if x > dimensions.1 {
                dimensions.1 = x;
            }
            if y > dimensions.3 {
                dimensions.3 = y
            }
        });
    });

    // Part 1
    let mut unit_count: usize = 0;
    let mut abyss_reached: bool = false;
    while !abyss_reached {
        unit_count += 1;
        let mut sand_position: Position = (500, 0);
        let mut sand_falling: bool = true;
        while sand_falling {
            let below_position: Position = (sand_position.0, sand_position.1 + 1);
            let below_left_position: Position = (sand_position.0 - 1, sand_position.1 + 1);
            let below_right_position: Position = (sand_position.0 + 1, sand_position.1 + 1);
            if occupation.get(&below_position).is_some() {
                if occupation.get(&below_left_position).is_some() {
                    if occupation.get(&below_right_position).is_some() {
                        sand_falling = false;
                    } else {
                        sand_position.0 += 1;
                    }
                } else {
                    sand_position.0 -= 1;
                }
            } else {
                sand_position.1 += 1;
            }

            if sand_position.1 > dimensions.3 {
                sand_falling = false;
                abyss_reached = true;
            }
        }
        occupation.insert(sand_position, Occupation::Sand);
    }
    println!("Part One: {}", unit_count - 1);

    // // Draw
    // for y in dimensions.2..dimensions.3 + 1 {
    //     for x in dimensions.0..dimensions.1 + 1 {
    //         match occupation.get(&(x, y)) {
    //             Some(Occupation::Air) => print!("."),
    //             Some(Occupation::Rock) => print!("#"),
    //             Some(Occupation::Sand) => print!("o"),
    //             None => print!("."),
    //         }
    //     }
    //     print!("\n");
    // }
}
