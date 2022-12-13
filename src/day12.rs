use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

pub type NodeIndex = usize;
pub type NodeHeight = usize;

#[derive(Debug)]
struct Graph {
    start: NodeIndex,
    end: NodeIndex,
    nodes: Vec<NodeHeight>,
    vertices: Vec<Vec<Vertex>>,
}

#[derive(Debug)]
struct Vertex {
    to_index: NodeIndex,
    elevation: i64,
}

fn main() {
    let file = File::open("./data/day12.input").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let mut graph = Graph::new(&input[..]);

    let result = graph.compute_shortest_distance_to_end();
    println!("Part 1: {:?}", result);

    // part 2
    graph.add_new_source();

    let result2 = graph.compute_shortest_distance_to_end() - 1;
    println!("Part 2: {:?}", result2);

}

impl Graph {
    fn new(lines: &[String]) -> Self {
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut start = 0;
        let mut end = 0;

        let expand_char_with_height = |character| -> (char, usize) {
            match character {
                a if alphabet.contains(&a) => alphabet
                    .iter()
                    .enumerate()
                    .find(|alphabet_char| *alphabet_char.1 == a)
                    .map(|(a, b)| (*b, a))
                    .unwrap(),
                'S' => ('S', 0),
                'E' => ('E', 25),
                _ => panic!("Score error"),
            }
        };

        let mut nodes: Vec<NodeHeight> = vec![];
        let mut vertices: Vec<Vec<Vertex>> = vec![];

        let width = lines.first().unwrap().len();
        let lines_iter = lines.iter().enumerate();
        for (vertical_index, line) in lines_iter {
            let previous_line = match vertical_index {
                0 => None,
                _ => lines.get(vertical_index - 1),
            };
            let next_line = lines.get(vertical_index + 1);
            for (horizontal_index, (char, height)) in
                line.chars().map(expand_char_with_height).enumerate()
            {
                let node_index = nodes.len();
                // mark start or end node
                match char {
                    'S' => start = node_index,
                    'E' => end = node_index,
                    _ => {}
                };

                // create vertices
                vertices.push(vec![]);
                // add node
                nodes.push(height);

                // Upper neighbour
                if let Some(previous_line) = previous_line {
                    let upper_neighbour = previous_line
                        .chars()
                        .map(expand_char_with_height)
                        .nth(horizontal_index)
                        .unwrap();
                    let elevation = upper_neighbour.1 as i64 - height as i64;
                    vertices[node_index].push(Vertex {
                        to_index: node_index - width,
                        elevation,
                    });
                };

                // Bottom neighbour
                if let Some(next_line) = next_line {
                    let bottom_neighbour = next_line
                        .chars()
                        .map(expand_char_with_height)
                        .nth(horizontal_index)
                        .unwrap();
                    let elevation = bottom_neighbour.1 as i64 - height as i64;
                    vertices[node_index].push(Vertex {
                        to_index: node_index + width,
                        elevation,
                    });
                }

                if let Some(left_neighbour) = match horizontal_index {
                    0 => None,
                    _ => line
                        .chars()
                        .map(expand_char_with_height)
                        .nth(horizontal_index - 1),
                } {
                    let elevation = left_neighbour.1 as i64 - height as i64;
                    vertices[node_index].push(Vertex {
                        to_index: node_index - 1,
                        elevation,
                    });
                };
                if let Some(right_neighbour) = line
                    .chars()
                    .map(expand_char_with_height)
                    .nth(horizontal_index + 1)
                {
                    let elevation = right_neighbour.1 as i64 - height as i64;
                    vertices[node_index].push(Vertex {
                        to_index: node_index + 1,
                        elevation,
                    });
                };
            }
        }

        Graph {
            nodes,
            vertices: vertices
                .into_iter()
                .map(|vertices_at_index| {
                    vertices_at_index
                        .into_iter()
                        .filter(|v| v.elevation <= 1)
                        .collect::<Vec<Vertex>>()
                })
                .collect(),
            start,
            end,
        }
    }

    fn add_new_source(&mut self) {
        self.start = self.nodes.len();
        self.nodes.push(0);

        let mut new_edges = vec![];
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, height)| **height == 0)
            .skip(1)
            .for_each(|(index, _)| {
                // create a vertex from 0 to index 
                new_edges.push(Vertex {
                    to_index: index,
                    elevation: 0,
                });
            });

        self.vertices.push(new_edges);
    }

    fn compute_shortest_distance_to_end(&self) -> usize {
        let mut distance_to_end: Vec<usize> = vec![usize::MAX; self.nodes.len()];
        let mut visited_node_indices: HashSet<NodeIndex> = HashSet::new();

        // let start_node_height = self.nodes[self.start];
        // mark the start node
        distance_to_end[self.start] = 0;

        for _ in 0..self.nodes.len() {
            let (current_index, _) = distance_to_end
                .iter()
                .enumerate()
                .filter(|(index, _)| !visited_node_indices.contains(index))
                .min_by(|a, b| a.1.cmp(b.1))
                .unwrap();

            // mark as visited
            visited_node_indices.insert(current_index);

            self.vertices[current_index].iter().for_each(|vertex| {
                if distance_to_end[current_index].saturating_add(1)
                    < distance_to_end[vertex.to_index]
                {
                    distance_to_end[vertex.to_index] =
                        distance_to_end[current_index].saturating_add(1)
                }
            });
        }

        distance_to_end[self.end]
    }
}
