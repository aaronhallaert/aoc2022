use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Move {
    direction: String,
    amount: i32,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position(i32, i32);

const ROPE_SIZE: usize = 10;

fn main() {
    let file = File::open("./data/day9.input").expect("file not found!");
    let moves = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
        .iter()
        .map(|s| {
            let (direction, amount) = s.split_once(' ').unwrap();
            Move {
                direction: direction.to_string(),
                amount: amount.parse().unwrap(),
            }
        })
        .collect::<Vec<Move>>();

    let mut visited_positions: HashSet<Position> = HashSet::new();

    // all knots are on the same position
    let rope: Vec<Position> = vec![Position(0, 0); ROPE_SIZE];

    // for every move, move the head and let the tails follow, returning a new rope state
    moves.iter().fold(
        rope,
        |mut rope_after_complete_move, move_action| -> Vec<Position> {
            // take the head of the rope
            let mut new_head = rope_after_complete_move[ROPE_SIZE - 1].clone();

            // repeat the move head and tail sequence X amount of times according to the input
            for _ in 0..move_action.amount {

                // println!("Old head {:?}", new_head);
                // move head
                new_head = match &move_action.direction[..] {
                    "U" => Position(new_head.0, new_head.1 + 1),
                    "D" => Position(new_head.0, new_head.1 - 1),
                    "R" => Position(new_head.0 + 1, new_head.1),
                    "L" => Position(new_head.0 - 1, new_head.1),
                    _ => panic!("Not a valid move"),
                };
                rope_after_complete_move[ROPE_SIZE - 1] = new_head.clone();
                // println!("New head {:?}", new_head);

                for i in (0..(ROPE_SIZE - 1)).rev() {
                    // take the head which is already moved
                    let moved_head = rope_after_complete_move[i + 1].clone();
                    // take the tail which needs to be updated at index i
                    let mut following_tail = rope_after_complete_move[i].clone();

                    // println!("Old tail {:?}", following_tail);
                    // where is the head in comparison to the tail
                    let movement_direction = (
                        moved_head.0 - following_tail.0,
                        moved_head.1 - following_tail.1,
                    );

                    // println!("ABS movement {}, {}", movement_direction.0.abs(), movement_direction.1.abs());
                    // println!("Movement {}, {}", movement_direction.0, movement_direction.1);
                    // update the following tail
                    following_tail = match movement_direction {
                        (a, b) if a.abs() < 2 && b.abs() < 2 => {
                            Position(following_tail.0, following_tail.1)
                        }
                        // up
                        (a, b) if a == 0 && b > 0 => {
                            // println!("Going up");
                            Position(following_tail.0, following_tail.1 + 1)
                        }
                        // down
                        (a, b) if a == 0 && b < 0 => {
                            // println!("Going down");
                            Position(following_tail.0, following_tail.1 - 1)
                        }
                        // left
                        (a, b) if a < 0 && b == 0 => {
                            // println!("Going left");
                            Position(following_tail.0 - 1, following_tail.1)
                        }
                        // right
                        (a, b) if a > 0 && b == 0 => {
                            // println!("Going right");
                            Position(following_tail.0 + 1, following_tail.1)
                        }
                        // right up
                        (a, b) if a > 0 && b > 0 => {
                            // println!("Going right up");
                            Position(following_tail.0 + 1, following_tail.1 + 1)
                        }
                        // right down
                        (a, b) if a > 0 && b < 0 => {
                            // println!("Going right down");
                            Position(following_tail.0 + 1, following_tail.1 - 1)
                        }
                        // left up
                        (a, b) if a < 0 && b > 0 => {
                            // println!("Going left up");
                            Position(following_tail.0 - 1, following_tail.1 + 1)
                        }
                        // left down
                        (a, b) if a < 0 && b < 0 => {
                            // println!("Going left down");
                            Position(following_tail.0 - 1, following_tail.1 - 1)
                        }
                        _ => {
                            panic!("SHIIT");
                        }
                    };

                    // println!("New tail {:?}", following_tail);
                    // println!("------");

                    // update the rope
                    rope_after_complete_move[i] = following_tail.clone();

                    if i == 0 {
                        // save tails position
                        visited_positions.insert(following_tail.clone());
                    };
                }
            }

            rope_after_complete_move.clone()
        },
    );

    println!("Visited positions: {:?}", visited_positions.len());
}
