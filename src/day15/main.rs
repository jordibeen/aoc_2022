use geo::geometry::Coord;
use geo::{BooleanOps, CoordsIter, Intersects, LineString, MultiPolygon, Polygon, Rect};
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

    // Part 1
    let mut dimensions: Dimensions = get_dimensions(&distances);
    let mut no_beacons: HashSet<Position> = HashSet::new();
    for x in dimensions.0..dimensions.1 {
        let pos: Position = (x, 2000000);
        if check_empty_space(&distances, &pos) && !beacons.contains(&pos) {
            no_beacons.insert(pos);
        }
    }
    println!("Part One: {:?}", no_beacons.len());

    // Part Two
    dimensions = (0, 4000000, 0, 4000000);
    let beacon_location: Coord = find_hidden_beacon(&distances, &dimensions);
    let pt2_answer = (beacon_location.x as usize * 4000000) + beacon_location.y as usize;
    println!("Part Two: {:?}", pt2_answer);
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

fn check_empty_space(distances: &HashMap<Position, usize>, pos: &Position) -> bool {
    for (sensor, distance) in distances {
        if (sensor.0.abs_diff(pos.0) + sensor.1.abs_diff(pos.1)) <= *distance {
            return true;
        }
    }
    false
}

fn find_hidden_beacon(distances: &HashMap<Position, usize>, dimensions: &Dimensions) -> Coord {
    // Create a Grid MultiPolygon containing dimensions
    let dim: Rect = Rect::new(
        Coord {
            x: dimensions.0 as f64,
            y: dimensions.2 as f64,
        },
        Coord {
            x: dimensions.1 as f64,
            y: dimensions.3 as f64,
        },
    );
    let grid: MultiPolygon = MultiPolygon::new(vec![dim.to_polygon()]);

    // Create a MultiPolygon containing all Sensor Views (empty spaces)
    let mut sensor_polygons: Vec<Polygon> = Vec::new();
    distances.iter().for_each(|(sensor, distance)| {
        let dist = *distance as isize;
        let top: Coord = Coord {
            x: sensor.0 as f64,
            y: (sensor.1 - dist) as f64,
        };
        let right: Coord = Coord {
            x: (sensor.0 + dist) as f64,
            y: sensor.1 as f64,
        };
        let bottom: Coord = Coord {
            x: sensor.0 as f64,
            y: (sensor.1 + dist) as f64,
        };
        let left: Coord = Coord {
            x: (sensor.0 - dist) as f64,
            y: sensor.1 as f64,
        };

        let linestring: LineString = LineString::new(vec![top, right, bottom, left]);
        let poly: Polygon = Polygon::new(linestring, vec![]);
        sensor_polygons.push(poly);
    });
    let sensor_view: MultiPolygon = MultiPolygon::new(sensor_polygons);

    // Intersect Grid and Sensor views
    let grid_sensor_intersection = &grid.intersection(&sensor_view);

    // Loop through the intersection's exterior coords, and create a Vector of neighbouring
    // Coords that ARE NOT in the Sensor View but DO fall within the grid's boundaries
    let mut hidden: Vec<Coord> = Vec::new();
    grid_sensor_intersection
        .exterior_coords_iter()
        .for_each(|exterior_chord| {
            let top: Coord = Coord {
                x: exterior_chord.x,
                y: (exterior_chord.y - 1 as f64),
            };
            let right: Coord = Coord {
                x: (exterior_chord.x + 1 as f64),
                y: exterior_chord.y,
            };
            let bottom: Coord = Coord {
                x: exterior_chord.x,
                y: (exterior_chord.y + 1 as f64),
            };
            let left: Coord = Coord {
                x: (exterior_chord.x - 1 as f64),
                y: exterior_chord.y,
            };

            if !sensor_view.intersects(&top) && grid.intersects(&top) {
                hidden.push(top);
            }
            if !sensor_view.intersects(&right) && grid.intersects(&right) {
                hidden.push(right);
            }
            if !sensor_view.intersects(&bottom) && grid.intersects(&bottom) {
                hidden.push(bottom);
            }
            if !sensor_view.intersects(&left) && grid.intersects(&left) {
                hidden.push(left);
            }
        });

    // We've probably found the hidden point multiple times, as it's neighbouring both top, right, bottom and left
    // exterior points, dedup it
    hidden.dedup();

    // Finally, return the hidden point that we've found
    hidden[0]
}
