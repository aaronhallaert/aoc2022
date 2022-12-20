use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Graph {
    tunnels: HashMap<String, Vec<String>>,
    valves: HashMap<String, usize>,
}

impl Graph {
    fn new(input: &[String]) -> Self {
        let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();
        let mut valves: HashMap<String, usize> = HashMap::new();
        input.iter().for_each(|s| {
            let (valve_desc, tunnel_desc) = s.split_once(';').unwrap();

            let valve_name = valve_desc.split(' ').nth(1).unwrap();
            let (_, rate_str) = valve_desc.split_once('=').unwrap();
            let rate: usize = rate_str.parse().unwrap();

            let tunnels_str = match tunnel_desc.split_once("valves ") {
                Some((_, tunnel_valves)) => tunnel_valves,
                None => tunnel_desc.split_once("valve ").unwrap().1,
            };

            let tunnels_for_valve: Vec<String> =
                tunnels_str.split(", ").map(|s| s.to_owned()).collect();

            valves.insert(valve_name.to_owned(), rate);
            tunnels.insert(valve_name.to_owned(), tunnels_for_valve);
        });

        Graph { tunnels, valves }
    }
}

fn main() {
    let file = File::open("./data/day16.test").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let graph = Graph::new(&input);

    println!("Graph: {:?}", graph);
}
