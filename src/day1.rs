use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("./data/day1.input").expect("file not found!");
    let buf_reader = io::BufReader::new(file);

    let mut calories: Vec<u32> = Vec::new();

    let subtotal = 0;
    buf_reader.lines().flatten().fold(subtotal, |subtotal, line| {
        if let Ok(calorie) = line.parse::<u32>() {
            subtotal + calorie
        } else {
            calories.push(subtotal);
            0
        }
    });

    println!("Part 1: {}", calories.iter().max().unwrap());
    calories.sort_by(|a,b| b.cmp(a));
    println!("Part 2: {:?}", calories.iter().take(3).sum::<u32>());
}
