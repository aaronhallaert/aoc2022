use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position(usize, usize);

impl Position {
    fn fall_down(&mut self) {
        self.1 += 1;
    }

    fn peek_fall_down(&self) -> Position {
        Position(self.0, self.1 + 1)
    }

    fn peek_fall_right(&self) -> Position {
        Position(self.0 + 1, self.1 + 1)
    }

    fn fall_right(&mut self) {
        self.1 += 1;
        self.0 += 1;
    }

    fn fall_left(&mut self) {
        self.1 += 1;
        self.0 -= 1;
    }

    fn peek_fall_left(&self) -> Position {
        Position(self.0 - 1, self.1 + 1)
    }

    fn reset(&mut self) {
        self.0 = 500;
        self.1 = 0;
    }

    fn is_in_range(&self, min_x: usize, max_x: usize) -> bool {
        min_x <= self.0 && max_x >= self.1
    }
}

fn main() {
    let file = File::open("./data/day14.input").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let mut occupied_positions: HashSet<Position> = HashSet::new();
    input.iter().for_each(|line_spec| {
        let points = line_spec
            .split(" -> ")
            .map(|point| {
                let (x, y) = point.split_once(',').unwrap();
                Position(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<Vec<Position>>();

        points.windows(2).for_each(|window| {
            assert_eq!(window.len(), 2, "Window should be of length 2");
            let point_a = &window[0];
            let point_b = &window[1];

            match (point_a, point_b) {
                (Position(a1, a2), Position(b1, b2)) if a1 == b1 && b2 > a2 => {
                    for n in *a2 as u64..(*b2 as u64 + 1) {
                        occupied_positions.insert(Position(*a1, n as usize));
                    }
                }
                (Position(a1, a2), Position(b1, b2)) if a1 == b1 && b2 < a2 => {
                    for n in *b2 as u64..(*a2 as u64 + 1) {
                        occupied_positions.insert(Position(*a1, n as usize));
                    }
                }
                (Position(a1, a2), Position(b1, b2)) if a2 == b2 && b1 > a1 => {
                    for n in *a1 as u64..(*b1 as u64 + 1) {
                        occupied_positions.insert(Position(n as usize, *a2));
                    }
                }
                (Position(a1, a2), Position(b1, b2)) if a2 == b2 && b1 < a1 => {
                    for n in *b1 as u64..(*a1 as u64 + 1) {
                        occupied_positions.insert(Position(n as usize, *a2));
                    }
                }
                _ => {
                    panic!("Could not draw a line")
                }
            }
        });
    });

    // find min x and max x
    let max_x = occupied_positions.iter().map(|p| p.0).max().unwrap();
    let min_x = occupied_positions.iter().map(|p| p.0).min().unwrap();

    // Sand is pouring from (500, 0)
    let mut falling_sand = Position(500, 0);
    let mut sand_unit = 1;
    'new_sand: loop {
        'sand_fall: loop {
            if !occupied_positions.contains(&falling_sand.peek_fall_down()) {
                falling_sand.fall_down();
            } else if !occupied_positions.contains(&falling_sand.peek_fall_left()) {
                falling_sand.fall_left();
            } else if !occupied_positions.contains(&falling_sand.peek_fall_right()) {
                falling_sand.fall_right();
            } else {
                // sand stay put
                occupied_positions.insert(falling_sand.clone());

                break 'sand_fall;
            }

            // check if the new position is still in range
            if !falling_sand.is_in_range(min_x, max_x) {
                sand_unit -= 1;
                break 'new_sand;
            };
        }

        sand_unit += 1;
        falling_sand.reset();
    }

    println!("{:?}", occupied_positions);
    println!("Min x: {:?}", min_x);
    println!("Max x: {:?}", max_x);
    println!("Result part 1: {}", sand_unit);
}
