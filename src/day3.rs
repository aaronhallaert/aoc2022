use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let calculate_score_for_char = |c: &char| {
        alphabet
            .iter()
            .enumerate()
            .find(|alphabet_char| *alphabet_char.1 == *c)
            .map_or_else(|| panic!("No score found for character {}", c), |found| found.0 + 1)
    };

    let find_overlapping_char = |vectors: &Vec<String>| {
        match vectors.len() {
            0 | 1 => {panic!("At least 2 strings should be passed")},
            _ => {}
        }

        vectors[0]
            .chars()
            .find(|c| vectors.iter().skip(1).all(|v| v.contains(*c)))
            .unwrap_or_else(|| panic!("No overlapping character found for {:?}", vectors))
    };

    #[allow(clippy::expect_used)]
    let file = File::open("./data/day3.input").expect("file not found!");

    let buf_reader = io::BufReader::new(file);
    let input = buf_reader.lines().flatten().collect::<Vec<String>>();

    // Part 1
    let rucksacks_input: Vec<(&str, &str)> =
        input.iter().map(|s| s.split_at(s.len() / 2)).collect();

    let score: usize = rucksacks_input
        .iter()
        .map(|(a, b)| find_overlapping_char(&vec![(*a).to_string(), (*b).to_string()]))
        .map(|c| calculate_score_for_char(&c))
        .sum();

    println!("Part 1: {:?}", score);

    // Part 2
    let chunks: Vec<Vec<String>> = input.chunks(3).map(std::convert::Into::into).collect();

    let badge_score: usize = chunks
        .iter()
        .map(|elves| find_overlapping_char(elves))
        .map(|c| calculate_score_for_char(&c))
        .sum();

    println!("Part 2: {:?}", badge_score);
}
