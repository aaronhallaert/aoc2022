use std::collections::HashSet;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("./data/day4.input").expect("file not found!");

    let buf_reader = io::BufReader::new(file);
    let input = buf_reader.lines().flatten().collect::<Vec<String>>();

    let parse_assignments = |assignment: &str| {
      let (a_start_string, a_end_string) = assignment.split_once('-').unwrap_or_else(|| panic!("Could not parse assignment"));

      (a_start_string.parse::<u32>().unwrap()..a_end_string.parse::<u32>().unwrap()+1)
          .collect::<HashSet<_>>()
    };


    let overlapping = input.iter()
         .map(|s| s.split_once(',').unwrap_or_else(|| panic!("Could not find assignments")))
         .filter(|(a,b)| {
             let assignment_a = parse_assignments(a);
             let assignment_b = parse_assignments(b);

             // PART 1 : all
             // PART 2 : any
             assignment_a.iter().any(|item| assignment_b.contains(item)) || assignment_b.iter().any(|item| assignment_a.contains(item))
         }).count();

    println!("Overlapping: {}", overlapping);

}
