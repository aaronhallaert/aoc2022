use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct TunnelSystem {
    tunnels: HashMap<String, Vec<String>>,
    valves: HashMap<String, usize>,
    /// distance from Valve source to Valve destination
    distances: HashMap<(String, String), usize>,
    /// indices of valves
    indices: HashMap<String, usize>,
}

#[derive(Hash, Debug, Eq, PartialEq)]
struct CacheEntry {
    time: isize,
    valve: usize,
    opened_valves: usize,
}

impl TunnelSystem {
    fn new(input: &[String]) -> Self {
        let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();
        let mut valves: HashMap<String, usize> = HashMap::new();
        let mut indices: HashMap<String, usize> = HashMap::new();
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

        valves.iter().enumerate().for_each(|(i, valve)| {
            indices.insert(valve.0.to_string(), i);
        });

        TunnelSystem {
            tunnels,
            valves,
            distances: HashMap::new(),
            indices,
        }
    }

    /// applies Floyd-Warshall to the graph (removes unreachable connections)
    fn compute_distances(&mut self) {
        self.tunnels.iter().for_each(|(source, destinations)| {
            destinations.iter().for_each(|dest| {
                self.distances
                    .insert((source.to_owned(), dest.to_owned()), 1);
            });
        });

        self.valves.iter().for_each(|(valve, _rate)| {
            self.distances
                .insert((valve.to_owned(), valve.to_owned()), 0);
        });

        self.valves.iter().for_each(|(valve, _)| {
            self.valves.iter().for_each(|(source, _)| {
                self.valves.iter().for_each(|(destination, _)| {
                    let test_distance = self
                        .distances
                        .get(&(source.to_owned(), valve.to_owned()))
                        .unwrap_or(&usize::MAX)
                        .saturating_add(
                            *self
                                .distances
                                .get(&(valve.to_owned(), destination.to_owned()))
                                .unwrap_or(&usize::MAX),
                        );
                    if self
                        .distances
                        .get(&(source.to_owned(), destination.to_owned()))
                        .unwrap_or(&usize::MAX)
                        > &test_distance
                    {
                        self.distances
                            .insert((source.to_owned(), destination.to_owned()), test_distance);
                    };
                });
            })
        });

        self.distances
            .retain(|k, _v| k.1 != "AA" && k.0 != k.1 && self.valves.get(&k.1).unwrap() != &0);
    }

    /// returns the max amount of pressure which can be released
    fn depth_first_search(
        &self,
        time: isize,
        current_valve: &str,
        mut opened_valves: usize,
        cache: &mut HashMap<CacheEntry, usize>,
    ) -> usize {
        let mut max_pressure = 0;

        let cache_key = CacheEntry {
            time,
            valve: *self.indices.get(current_valve).unwrap(),
            opened_valves,
        };

        if let Some(cached_value) = cache.get(&cache_key) {
            return *cached_value;
        };

        for ((s, neighbour), dist) in self
            .distances
            .iter()
            .filter(|((s, _), _)| s == current_valve)
        {
            if opened_valves & (1 << self.indices.get(neighbour).unwrap()) != 0 {
                continue;
            };

            let rem_time = time - 1 - *dist as isize;
            if rem_time <= 0 {
                continue;
            };

            opened_valves |= 1 << self.indices.get(s).unwrap();

            let additional_pressure = rem_time as usize * self.valves.get(neighbour).unwrap();

            max_pressure = cmp::max(
                max_pressure,
                self.depth_first_search(rem_time, neighbour, opened_valves, cache)
                    + additional_pressure,
            );
        }

        let new_cache_key = CacheEntry {
            time,
            valve: *self.indices.get(current_valve).unwrap(),
            opened_valves,
        };

        cache.insert(new_cache_key, max_pressure);
        max_pressure
    }
}

fn main() {
    let file = File::open("./data/day16.input").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let mut tunnel_system = TunnelSystem::new(&input);

    tunnel_system.compute_distances();

    println!("Computing result");
    // println!(
    //     "Result: {:?}",
    //     tunnel_system.depth_first_search(30, "AA", 0, &mut HashMap::new())
    // )

    let b: usize = (1 << tunnel_system.valves.len()) - 1;

    let mut m = 0;

    let mut cache = HashMap::new();

    println!("{}",b);
    for i in 0..b + 1 {
        m = cmp::max(m, tunnel_system.depth_first_search(26, "AA", i, &mut cache)
            + tunnel_system.depth_first_search(26, "AA", b ^ i, &mut cache));
    }

    println!("Result: {}", m);
    // println!("Graph: {:?}", tunnel_system);
}
