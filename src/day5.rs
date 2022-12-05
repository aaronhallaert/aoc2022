use std::{
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    let file = File::open("./data/day5.input").expect("file not found!");

    let buf_reader = io::BufReader::new(file);
    let input = buf_reader.lines().flatten().collect::<Vec<String>>();
    let mut input_split = input.split(std::string::String::is_empty);

    #[allow(clippy::unwrap_used)]
    let (mut stack, moves_input) = (
        input_split.next().unwrap().to_vec(),
        input_split.next().unwrap().to_vec(),
    );

    #[allow(clippy::unwrap_used)]
    let indices: Vec<usize> = stack
        .pop()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c.is_numeric() { Some(i) } else { None })
        .collect();

    let mut stacks = indices
        .iter()
        .map(|i| {
            let containers = stack
                .iter()
                .filter_map(|s| {
                    let chars = s.chars().collect::<Vec<char>>();
                    let container = chars[*i];
                    if container == ' ' {
                        None
                    } else {
                        Some(container)
                    }
                })
                .rev()
                .collect::<Vec<char>>();
            containers
        })
        .collect::<Vec<Vec<char>>>();

    let moves: Vec<Move> = moves_input
        .iter()
        .map(|s| Move::from_str(s).unwrap_or_else(|_| panic!("Could not parse move")))
        .collect();

    for move_action in &moves {
        let n = stacks[move_action.from].len();
        let containers_to_move: Vec<char> = stacks[move_action.from]
            .drain(n - move_action.amount..)
            .rev() // remove for PART 2
            .collect();

        stacks[move_action.to].extend(containers_to_move);
    }

    let result = stacks
        .iter()
        .filter_map(|s| s.iter().last())
        .collect::<String>();

    println!("Result: {:?}", result);
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numerics = s
            .split(' ')
            .filter_map(|string_split| string_split.parse::<usize>().ok());

        #[allow(clippy::unwrap_used)]
        Ok(Self {
            amount: numerics.next().unwrap(),
            from: numerics.next().unwrap() - 1,
            to: numerics.next().unwrap() - 1,
        })
    }
}
