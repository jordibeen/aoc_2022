use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

// Position is a tuple containing (x, y)-points
// Eg. (8, 7)
type Position = (isize, isize);

// Dimension is a tuple containing (x-min, x-max, y-min, y-max)
// Eg. (-8, 28, -10, 26)
type Dimensions = (isize, isize, isize, isize);

fn main() {
    println!("--- Day 15: Beacon Exclusion Zone ---");

    // Keeping track of distance that first Beacon is found from a Sensor
    // Eg. ((494,5), 9)
    let mut distances: HashMap<Position, usize> = HashMap::new();

    // Keeping track of Beacon Positions
    let mut beacons: Vec<Position> = Vec::new();

    let input: String = fs::read_to_string("./src/day15/input.txt").expect("File should exist");

    input.lines().for_each(|line| {
        let cap = Regex::new(
            r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)",
        ).unwrap().captures(line).unwrap();
        let sensor_x = cap.name("sx").unwrap().as_str().parse::<isize>().unwrap();
        let sensor_y = cap.name("sy").unwrap().as_str().parse::<isize>().unwrap();
        let beacon_x = cap.name("bx").unwrap().as_str().parse::<isize>().unwrap();
        let beacon_y = cap.name("by").unwrap().as_str().parse::<isize>().unwrap();

        let sensor: Position = (sensor_x, sensor_y);
        let beacon: Position = (beacon_x, beacon_y);
        beacons.push(beacon);

        let distance: usize = ((beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs()) as usize;
        distances.insert(sensor, distance);
    });

    let dimensions: Dimensions = get_dimensions(&distances);

    let mut no_beacons: HashSet<Position> = HashSet::new();
    for x in dimensions.0..dimensions.1 {
        let pos: Position = (x, 2000000);
        if check_empty_space(&distances, &beacons, &pos) {
            no_beacons.insert(pos);
        }
    }
    println!("Part One: {:?}", no_beacons.len());
}

fn get_dimensions(distances: &HashMap<Position, usize>) -> Dimensions {
    let mut dim: Dimensions = (0, 0, 0, 0);

    distances.iter().for_each(|(sensor, distance)| {
        let sensor_x_min = sensor.0 - *distance as isize;
        let sensor_x_max = sensor.0 + *distance as isize;
        let sensor_y_min = sensor.1 - *distance as isize;
        let sensor_y_max = sensor.1 + *distance as isize;

        if sensor_x_min < dim.0 {
            dim.0 = sensor_x_min
        }
        if sensor_x_max > dim.1 {
            dim.1 = sensor_x_max;
        }
        if sensor_y_min < dim.2 {
            dim.2 = sensor_y_min;
        }
        if sensor_y_max > dim.3 {
            dim.3 = sensor_y_max
        }
    });

    dim
}

fn check_empty_space(
    distances: &HashMap<Position, usize>,
    beacons: &Vec<Position>,
    pos: &Position,
) -> bool {
    for (sensor, distance) in distances {
        if ((sensor.0.abs_diff(pos.0) + sensor.1.abs_diff(pos.1)) <= *distance)
            && !beacons.contains(&pos)
        {
            return true;
        }
    }
    false
}
