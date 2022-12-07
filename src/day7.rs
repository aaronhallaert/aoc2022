use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let file = File::open("./data/day7.input").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let mut directories: HashMap<String, usize> = HashMap::new();

    let file_regex = Regex::new(r"([0-9]+) ([\w\.]+)$").unwrap();
    let dir_regex = Regex::new(r"dir (\w+)$").unwrap();
    let ls_regex = Regex::new(r"\$ ls").unwrap();
    let cd_regex = Regex::new(r"\$ cd ([\w/]+)$").unwrap();
    let cd_back_regex = Regex::new(r"\$ cd \.\.$").unwrap();

    let mut location_helper: Vec<String> = Vec::new();
    let mut location_history: Vec<Vec<String>> = input
        .iter()
        .filter(|l| cd_regex.is_match(l) || cd_back_regex.is_match(l))
        .map(|l| {
            if cd_regex.is_match(l) {
                let dir = &cd_regex.captures(l).unwrap()[1];
                match dir {
                    "/" => {
                        location_helper.push(String::new());
                    }
                    _ => {
                        location_helper.push(dir.to_string());
                    }
                }
            } else {
                location_helper.pop();
            }
            location_helper.clone()
        })
        .collect();

    location_history.reverse();
    let mut location_iterator = location_history.iter();

    let final_location = vec!["/".to_string()];

    input.iter().rev().fold(
        (0, location_iterator.next()),
        |(size, location), line| {
            //println!("LINE TO PARSE: {:?}", line);
            if file_regex.is_match(line) {
                //println!("\tLine is file");
                let file_size_str = &file_regex.captures(line).unwrap()[1];
                let file_size = file_size_str.parse::<usize>().unwrap();

                (size + file_size, location)
            } else if dir_regex.is_match(line) {
                let directory_name = &dir_regex.captures(line).unwrap()[1];
                let loc = location.unwrap().join("/");

                let key = vec![&loc[..], directory_name].join("/");
                (size + directories[&key], location)
            } else if ls_regex.is_match(line) {
                let key = location.unwrap().join("/");
                directories.insert(key, size);

                (0, location)
            }
            else if cd_regex.is_match(line) || cd_back_regex.is_match(line) {
                let loc = location_iterator.next().unwrap_or(&final_location);

                (0, Some(loc))
            } else {
                panic!("Should not occur")
            }
        },
    );

    // for (key, value) in &directories {
        //println!("{:?}: {:?}", key, value);
    // }


    let small_dirs = directories
        .iter()
        .filter(|d| *d.1 < 100_000)
        .map(|d| d.0.to_string())
        .collect::<Vec<String>>();
    let result: usize = small_dirs.iter().map(|d| directories[d]).sum();

    let required_space = 30_000_000;
    let total_space =  70_000_000;
    let used_space = directories[""];
    let space_to_delete = required_space - (total_space - used_space);

    let mut dir_vector: Vec<(String, usize)>= directories.into_iter().collect();
    dir_vector.sort_by(|a, b| a.1.cmp(&b.1));
    let delete_size = dir_vector.iter().find(|(_key, value)| {
        value > &space_to_delete
    }).unwrap().1;


    println!("Part 1 {:?}", result);
    println!("Part 2 {:?}", delete_size);
}
