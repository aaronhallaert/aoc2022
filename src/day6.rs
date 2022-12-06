use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

const SIZE: usize = 14;

fn main() {
    let file = File::open("./data/day6.input").expect("file not found!");
    let input = &(io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>())[0];

    let characters = input.chars().collect::<Vec<char>>();
    let (result, _window) = characters
        .windows(SIZE)
        .enumerate()
        .find(|(_index, win)| {
            let subwindow = *win;
            let mut collection: HashSet<char> = HashSet::new();
            collection.extend(subwindow);

            //collection.len() == 4
            collection.len() == SIZE
        })
        .unwrap();

    println!("Result {}", result + SIZE);
}
