use std::collections::HashMap;
use std::fs;

// Position is a tuple containing (x, y)-points
// Eg. (494, 7)
type Position = (usize, usize);

// Dimension is a tuple containing (x-min, x-max, y-min, y-max)
// Eg. (480, 520, 0, 9)
type Dimension = (usize, usize, usize, usize);

#[derive(Clone, Debug)]
enum Occupation {
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
    let mut dimension: Dimension = (0, 0, 0, 0);

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
            if x < dimension.0 || dimension.0 == 0 {
                dimension.0 = x;
            }
            if x > dimension.1 {
                dimension.1 = x;
            }
            if y > dimension.3 {
                dimension.3 = y
            }
        });
    });

    // Copy initial occuation state for part 2
    let mut occupation_pt2: HashMap<Position, Occupation> = HashMap::new();
    occupation_pt2.clone_from(&occupation);

    // Part 1
    let pt1 = pour_sand(&mut occupation, dimension, false);
    println!("Part One: {}", pt1);

    // Part 2
    dimension.0 = 0;
    dimension.1 = 1000;
    dimension.3 += 2;
    (dimension.0..dimension.1).for_each(|x| {
        occupation_pt2.insert((x, dimension.3), Occupation::Rock);
    });

    let pt2: usize = pour_sand(&mut occupation_pt2, dimension, true);
    println!("Part Two: {}", pt2);
}

fn pour_sand(
    occupation: &mut HashMap<Position, Occupation>,
    dimension: Dimension,
    pt2: bool,
) -> usize {
    let mut unit_count: usize = 0;
    let mut abyss_reached: bool = false;
    while !abyss_reached {
        unit_count += 1;
        let mut sand_position: Position = (500, 0);
        let mut sand_falling: bool = true;
        while sand_falling {
            if pt2 {
                if occupation.get(&(500, 0)).is_some() {
                    sand_falling = false;
                    abyss_reached = true;
                };
            }
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

            if !pt2 {
                if sand_position.1 > dimension.3 {
                    sand_falling = false;
                    abyss_reached = true;
                }
            }
        }
        occupation.insert(sand_position, Occupation::Sand);
    }

    unit_count - 1
}
