use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Position(isize, isize);

#[derive(Debug)]
struct RowCoverage(isize, isize);

#[derive(Debug)]
struct Sensor {
    position: Position,
    closest_beacon: Position,
    coverage: HashMap<isize, RowCoverage>,
}

fn manhattan_distance(position_a: &Position, position_b: &Position) -> usize {
    (position_a.0 - position_b.0).unsigned_abs() + (position_a.1 - position_b.1).unsigned_abs()
}

// Sensor at x=2, y=18
fn get_coordinates(line: &str) -> Position {
    let mut line_iter = line.split('=');
    line_iter.next();
    let x_string = line_iter.next().unwrap().replace(", y", "");
    let y_string = line_iter.next().unwrap();

    Position(x_string.parse().unwrap(), y_string.parse().unwrap())
}

fn generate_coverage(
    center_position: &Position,
    manh_distance: isize,
) -> HashMap<isize, RowCoverage> {
    let mut coverage = HashMap::new();
    for row_index in center_position.1 - manh_distance..center_position.1 + manh_distance + 1 {
        let horizontal_radius = manh_distance - (center_position.1 - row_index).abs();
        // println!("row_index {}", row_index);
        // println!("horizontal_radius {}", horizontal_radius);

        coverage.insert(
            row_index,
            RowCoverage(
                center_position.0 - horizontal_radius,
                center_position.0 + horizontal_radius,
            ),
        );
    }

    coverage
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor_desc, closest_beacon_desc) = s.split_once(':').unwrap();

        let sensor_coord = get_coordinates(sensor_desc);
        let beacon_coord = get_coordinates(closest_beacon_desc);
        // println!("Sensor: {:?}", &sensor_coord);
        // println!("Beacon: {:?}", &beacon_coord);

        let distance = manhattan_distance(&sensor_coord, &beacon_coord);
        // println!("Distance: {}", distance);

        let coverage = generate_coverage(&sensor_coord, distance as isize);
        // println!("Coverage: {:?}", coverage);

        Ok(Sensor {
            closest_beacon: beacon_coord,
            coverage,
            position: sensor_coord,
        })
    }
}

fn main() {
    let file = File::open("./data/day15.input").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    // let interested_row = 10;
    let interested_row = 2000000;

    let sensors = input
        .iter()
        .map(|i| Sensor::from_str(i).unwrap()).collect::<Vec<Sensor>>();

    let range = sensors
        .iter()
        .fold(HashSet::new(), |mut final_range, sensor| {
            if let Some(coverage_at_row) = sensor.coverage.get(&interested_row) {
                let range = coverage_at_row.0..coverage_at_row.1 + 1;
                final_range.extend(range);
            }
            final_range
        })
        .len();

    let beacons_on_row = sensors.iter().filter_map(|s| {
        if s.closest_beacon.1 == interested_row {
            Some(&s.closest_beacon)
        }
        else {
            None
        }
    }).unique().count();

    println!("Beacons on row: {}", beacons_on_row);

    println!("Range: {}", range - beacons_on_row);
}
