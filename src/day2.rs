use std::cmp::Ordering;

use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("./data/day2.input").expect("file not found!");
    let buf_reader = io::BufReader::new(file);

    let input: Vec<_> = buf_reader.lines().flatten().collect();

    let score: u32 = input
        .iter()
        .map(|l| Game::new(l, false))
        .map(|game| game.score())
        .sum();

    println!("Part 1 - score: {}", score);

    let score2: u32 = input
        .iter()
        .map(|l| Game::new(l, true))
        .map(|game| game.score())
        .sum();

    println!("Part 2 - score: {}", score2);
}

#[derive(PartialEq, Clone)]
enum Hand {
    Rock,
    Scissors,
    Paper,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (a, b) if a == b => Some(Ordering::Equal),
            (Hand::Rock, Hand::Scissors) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Rock) => Some(Ordering::Greater),
            (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Less),
        }
    }
}

impl Hand {
    fn value(&self) -> u32 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

struct Game {
    opponents_hand: Hand,
    my_hand: Hand,
}

impl Game {
    pub fn new(input_string: &str, generate_hand: bool) -> Game {
        let convert_hand = |hand| match hand {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => {
                panic!("Could not parse hands")
            }
        };

        let generated_hand = |opponent_hand, instruction| -> Option<Hand> {
            match instruction {
                "X" => vec![Hand::Rock, Hand::Paper, Hand::Scissors]
                    .into_iter()
                    .find(|h| h < opponent_hand),
                "Y" => vec![Hand::Rock, Hand::Paper, Hand::Scissors]
                    .into_iter()
                    .find(|h| h == opponent_hand),
                "Z" => vec![Hand::Rock, Hand::Paper, Hand::Scissors]
                    .into_iter()
                    .find(|h| h > opponent_hand),
                _ => {
                    panic!("Could not parse instruction")
                }
            }
        };

        let (opponents_hand, my_hand) = input_string.split_once(' ').unwrap();

        let opponents_hand = convert_hand(opponents_hand);
        let my_hand = match generate_hand {
            false => convert_hand(my_hand),
            true => generated_hand(&opponents_hand, my_hand).unwrap(),
        };

        Game {
            opponents_hand,
            my_hand,
        }
    }

    pub fn score(&self) -> u32 {
        match (&self.opponents_hand, &self.my_hand) {
            (a, b) if a > b => b.value(),
            (a, b) if a == b => b.value() + 3,
            (a, b) if b > a => b.value() + 6,
            _ => panic!("Big mistakes were made"),
        }
    }
}
