use std::{
    fs::File,
    io::{self, BufRead},
};

const ROUNDS: usize = 10000;

#[derive(Debug)]
enum Operation {
    Square,
    Double,
    Add(usize),
    Multiply(usize),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    test_division: usize,
    inspections: usize,
    operation: Operation,
    monkey_t: usize,
    monkey_f: usize,
}

fn main() {
    let file = File::open("./data/day11.input").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let mut monkeys = input
        .chunks(7)
        .map(|s| {
            let mut temp_s = s.to_vec();
            temp_s.retain(|s| !s.is_empty());
            Monkey::new(&temp_s.join("\n"))
        })
        .collect::<Vec<Monkey>>();

    let common_multiple = monkeys.iter().fold(1, |prev, m| prev * m.test_division) * 3;
    for _round in 0..ROUNDS {
        // loop over all monkeys
        for monkey_index in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_index];

            // (item, monkey_to_throw_to)
            let item_moves: Vec<(usize, usize)> = monkey.inspect(common_multiple);

            item_moves.iter().for_each(|(item, monkey_index)| {
                monkeys[*monkey_index].receive(*item);
            })
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    let result = monkeys[0].inspections * monkeys[1].inspections;
    println!("Result: {:?}", result);
}

impl Monkey {
    fn inspect(&mut self, worry_divider: usize) -> Vec<(usize, usize)> {
        self.items
            .drain(..)
            .map(|current_item| {
                // loop over each item of the monkey
                // pop an item from the monkey's items

                // add an inspection to the monkey
                self.inspections += 1;

                // do the operation on the item
                let new_item = match self.operation {
                    Operation::Square => current_item * current_item,
                    Operation::Double => current_item * 2,
                    Operation::Add(operand) => current_item + operand,
                    Operation::Multiply(operand) => current_item * operand,
                };

                // PART 1
                // let after_inspection_item = new_item / 3;
                // ----------------------------------------
                // PART 2
                let after_inspection_item = new_item % worry_divider;

                // perform the division check
                // insert new item to other monkey
                if after_inspection_item % self.test_division == 0 {
                    (after_inspection_item, self.monkey_t)
                } else {
                    (after_inspection_item, self.monkey_f)
                }
            })
            .collect()
    }

    fn receive(&mut self, item: usize) {
        self.items.push(item);
    }

    fn new(description: &str) -> Self {
        assert!(
            description.lines().count() == 6,
            "Could not parse the Monkey description, line numbers not correct"
        );

        let mut description_iter = description.lines();
        let _name: String = description_iter
            .next()
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .replace(':', "")
            .parse()
            .unwrap();

        let items = description_iter
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let operation_vec = description_iter
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .collect::<Vec<&str>>();

        let operation = match operation_vec[..] {
            ["old", "+", "old"] => Operation::Double,
            ["old", "*", "old"] => Operation::Square,
            ["old", "*", a] => Operation::Multiply(a.parse::<usize>().unwrap()),
            ["old", "+", a] => Operation::Add(a.parse().unwrap()),
            _ => {
                panic!("Can't parse operation")
            }
        };

        let test_division = description_iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let monkey_t = description_iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let monkey_f = description_iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            test_division,
            inspections: 0,
            operation,
            monkey_t,
            monkey_f,
        }
    }
}
