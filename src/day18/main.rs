use std::fs;

// Cube is a tuple containing (x, y, z)
type Cube = (isize, isize, isize);

fn main() {
    println!("--- Day 18: Boiling Boulders ---");

    let mut cubes: Vec<Cube> = Vec::new();

    let input: String = fs::read_to_string("./src/day18/input.txt").expect("File should exist");

    input.lines().for_each(|line| {
        let cube_data: Vec<isize> = line
            .split(",")
            .map(|val| val.parse::<isize>().unwrap())
            .collect();

        cubes.push((cube_data[0], cube_data[1], cube_data[2]));
    });

    let mut count: isize = 0;
    cubes.iter().for_each(|cube| {
        get_neighbours(cube).iter().for_each(|neighbour| {
            if !cubes.contains(neighbour) {
                count += 1;
            }
        });
    });

    println!("Part One: {:?}", count);
}

fn get_neighbours(cube: &Cube) -> Vec<Cube> {
    let mut neighbours: Vec<Cube> = Vec::new();

    neighbours.push((cube.0 + 1, cube.1, cube.2));
    neighbours.push((cube.0, cube.1 + 1, cube.2));
    neighbours.push((cube.0, cube.1, cube.2 + 1));
    neighbours.push((cube.0 - 1, cube.1, cube.2));
    neighbours.push((cube.0, cube.1 - 1, cube.2));
    neighbours.push((cube.0, cube.1, cube.2 - 1));

    neighbours
}
