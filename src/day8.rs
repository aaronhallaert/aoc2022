use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead}, time::Instant,
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position(usize, usize);

fn main() {
    let file = File::open("./data/day8large.input").expect("file not found!");
    let now = Instant::now();
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
        .iter()
        .map(|st| {
            st.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let width: usize = input.len();
    let height: usize = input[0].len();

    let mut visible_positions: HashSet<Position> = HashSet::new();
    let mut trees_scenic_score: HashMap<Position, usize> = HashMap::new();

    // left to right
    input.iter().enumerate().for_each(|(iter_height, row)| {
        row.iter().enumerate().fold(
            (0, vec![]),
            |(current_height, mut passed_trees), (iter_width, tree)| -> (u32, Vec<u32>) {
                let pos = Position(iter_height, iter_width);

                let current_score = trees_scenic_score.get(&pos).unwrap_or(&1);

                trees_scenic_score.insert(
                    pos.clone(),
                    current_score * compute_scenic_distance(&passed_trees[..], *tree),
                );

                passed_trees.push(*tree);

                if tree > &current_height || iter_width == 0 {
                    visible_positions.insert(pos);
                    return (*tree, passed_trees);
                };

                (current_height, passed_trees)
            },
        );
    });

    // right to left
    input.iter().enumerate().for_each(|(iter_height, row)| {
        row.iter().rev().enumerate().fold(
            (0, vec![]),
            |(current_height, mut passed_trees), (inv_iter_width, tree)| -> (u32, Vec<u32>) {
                let pos = Position(iter_height, (width - 1) - inv_iter_width);

                let current_score = trees_scenic_score.get(&pos).unwrap_or(&1);

                trees_scenic_score.insert(
                    pos.clone(),
                    current_score * compute_scenic_distance(&passed_trees[..], *tree),
                );

                passed_trees.push(*tree);

                if tree > &current_height || inv_iter_width == 0 {
                    visible_positions.insert(pos);
                    return (*tree, passed_trees);
                };

                (current_height, passed_trees)
            },
        );
    });

    // return for every width a vector of columns
    let transformed = (0..width)
        .collect::<Vec<usize>>()
        .iter()
        .map(|w| input.iter().map(|column| column[*w]).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    // top to bottom
    transformed
        .iter()
        .enumerate()
        .for_each(|(iter_width, column)| {
            column.iter().enumerate().fold(
                (0, vec![]),
                |(current_height, mut passed_trees), (iter_height, tree)| -> (u32, Vec<u32>) {
                    let pos = Position(iter_height, iter_width);

                    let current_score = trees_scenic_score.get(&pos).unwrap_or(&1);

                    trees_scenic_score.insert(
                        pos.clone(),
                        current_score * compute_scenic_distance(&passed_trees[..], *tree),
                    );

                    passed_trees.push(*tree);

                    if tree > &current_height || iter_height == 0 {
                        visible_positions.insert(pos);
                        return (*tree, passed_trees);
                    };

                    (current_height, passed_trees)
                },
            );
        });

    // bottom to top
    transformed
        .iter()
        .enumerate()
        .for_each(|(iter_width, column)| {
            column.iter().rev().enumerate().fold(
                (0, vec![]),
                |(current_height, mut passed_trees), (inv_iter_height, tree)| -> (u32, Vec<u32>) {
                    let pos = Position((height - 1) - inv_iter_height, iter_width);

                    let current_score = trees_scenic_score.get(&pos).unwrap_or(&1);

                    trees_scenic_score.insert(
                        pos.clone(),
                        current_score * compute_scenic_distance(&passed_trees[..], *tree),
                    );

                    passed_trees.push(*tree);

                    if tree > &current_height || inv_iter_height == 0 {
                        visible_positions.insert(pos);
                        return (*tree, passed_trees);
                    };

                    (current_height, passed_trees)
                },
            );
        });

    let result: usize = visible_positions.len();

    let max_scenic_distance = trees_scenic_score.values().max().unwrap();

    println!("Result: {:?}", result);
    println!("Part 2: {:?}", max_scenic_distance);
    println!("Elapsed time: {:?}", now.elapsed())
}

/// returns the amount of trees the passed tree can see
fn compute_scenic_distance(passed_trees: &[u32], tree: u32) -> usize {
    if passed_trees.is_empty() {
        return 0; 
    };

    let max = passed_trees.len() - 1;
    let res = passed_trees
        .iter()
        .rev()
        .enumerate()
        .find(|(_, t)| *t >= &tree)
        .unwrap_or((max, &0))
        .0 + 1;

    res
}

