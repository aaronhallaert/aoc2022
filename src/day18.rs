use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

#[derive(Debug)]
struct Cube {
    position: (isize, isize, isize),
    // top bottom left right front rear
    covered_side: Vec<bool>,
    // top bottom left right front rear
    free_space: Vec<bool>,
}

impl Cube {
    fn mark_top(&mut self, positions: &HashSet<(isize, isize, isize)>) {
        let position_to_check = (self.position.0 + 1, self.position.1, self.position.2);
        if positions.iter().any(|p| p == &position_to_check) {
            self.covered_side[0] = true
        } else if is_air_covered(&position_to_check, positions) {
            self.free_space[0] = false
        }
    }

    fn mark_bottom(&mut self, positions: &HashSet<(isize, isize, isize)>) {
        let position_to_check = (self.position.0 - 1, self.position.1, self.position.2);
        if positions.iter().any(|p| p == &position_to_check) {
            self.covered_side[1] = true
        } else if is_air_covered(&position_to_check, positions) {
            self.free_space[1] = false
        }
    }

    fn mark_left(&mut self, positions: &HashSet<(isize, isize, isize)>) {
        let position_to_check = (self.position.0, self.position.1 - 1, self.position.2);
        if positions.iter().any(|p| p == &position_to_check) {
            self.covered_side[2] = true
        } else if is_air_covered(&position_to_check, positions) {
            self.free_space[2] = false
        }
    }

    fn mark_right(&mut self, positions: &HashSet<(isize, isize, isize)>) {
        let position_to_check = (self.position.0, self.position.1 + 1, self.position.2);
        if positions.iter().any(|p| p == &position_to_check) {
            self.covered_side[3] = true
        } else if is_air_covered(&position_to_check, positions) {
            self.free_space[3] = false
        }
    }

    fn mark_front(&mut self, positions: &HashSet<(isize, isize, isize)>) {
        let position_to_check = (self.position.0, self.position.1, self.position.2 - 1);
        if positions.iter().any(|p| p == &position_to_check) {
            self.covered_side[4] = true
        } else if is_air_covered(&position_to_check, positions) {
            self.free_space[4] = false
        }
    }

    fn mark_rear(&mut self, positions: &HashSet<(isize, isize, isize)>) {
        let position_to_check = (self.position.0, self.position.1, self.position.2 + 1);
        if positions.iter().any(|p| p == &position_to_check) {
            self.covered_side[5] = true
        } else if is_air_covered(&position_to_check, positions) {
            self.free_space[5] = false
        }
    }
}

fn position_neighbours(position: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    vec![
        (position.0 + 1, position.1, position.2),
        (position.0 - 1, position.1, position.2),
        (position.0, position.1 + 1, position.2),
        (position.0, position.1 - 1, position.2),
        (position.0, position.1, position.2 + 1),
        (position.0, position.1, position.2 - 1),
    ]
}

fn is_air_covered(
    air_position: &(isize, isize, isize),
    positions: &HashSet<(isize, isize, isize)>,
) -> bool {
    let mut neighbouring_positions: HashSet<(isize, isize, isize)> = HashSet::new();
    neighbouring_positions.extend(position_neighbours(air_position));
    neighbouring_positions.iter().all(|p| positions.contains(p)) || positions.contains(air_position)
}

fn main() {
    let file = File::open("./data/day18.test").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let mut cubes: Vec<Cube> = input
        .iter()
        .map(|i| Cube {
            position: i
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap(),
            covered_side: vec![false; 6],
            free_space: vec![true; 6],
        })
        .collect();

    let cube_positions = cubes
        .iter()
        .map(|c| c.position)
        .collect::<HashSet<(isize, isize, isize)>>();

    cubes.iter_mut().for_each(|cube| {
        cube.mark_top(&cube_positions);
        cube.mark_bottom(&cube_positions);
        cube.mark_left(&cube_positions);
        cube.mark_right(&cube_positions);
        cube.mark_front(&cube_positions);
        cube.mark_rear(&cube_positions);
    });

    let result = cubes.iter().fold(0, |count, cube| {
        count + cube.covered_side.iter().filter(|s| !**s).count()
    });

    let part_2 = cubes.iter().fold(0, |count, cube| {
        count + cube.free_space.iter().filter(|s| **s).count()
    });

    println!("Result: {}", result);
    println!("Result 2: {}", part_2);
}
