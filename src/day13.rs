use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Group {
    data: Vec<Unit>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Unit {
    IntUnit(usize),
    ArrayUnit(Vec<Unit>),
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(Value::Array(v)) => {
                let data = v.iter().map(Unit::from_json_value).collect::<Vec<Unit>>();
                Ok(Group { data })
            }
            _ => Err(()),
        }
    }
}

impl Unit {
    fn from_json_value(s: &Value) -> Self {
        match s {
            Value::Number(v) => Unit::IntUnit(v.as_u64().unwrap() as usize),
            Value::Array(v) => {
                Unit::ArrayUnit(v.iter().map(Unit::from_json_value).collect::<Vec<Unit>>())
            }
            _ => panic!("Oeps, fucked up parsing the unit"),
        }
    }
}

fn main() {
    let file = File::open("./data/day13.input").expect("file not found!");
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let groups: Vec<Vec<String>> = input
        .chunks(3)
        .map(|s| {
            let mut temp_s = s.to_vec();
            temp_s.retain(|s| !s.is_empty());
            temp_s
        })
        .collect();

    // check ordering for each group
    let result = groups
        .iter()
        .enumerate()
        .map(|(group_index, groups)| {
            assert_eq!(groups.len(), 2);

            let unit_1: Group = Group::from_str(&groups[0]).unwrap();
            let unit_2: Group = Group::from_str(&groups[1]).unwrap();

            let ordered = match are_groups_ordered(&unit_1, &unit_2) {
                Some(r) => r,
                None => unit_1.data.len() < unit_2.data.len(),
            };

            (group_index, ordered)
        })
        .filter(|(_index, ordered)| {
            // println!("Index {} - Ordered {:?}", index + 1, ordered);
            *ordered
        })
        .map(|(index, _)| index + 1)
        .sum::<usize>();

    println!("Result: {}", result);

    let mut group_vector = groups
        .iter()
        .flatten()
        .map(|a| Group::from_str(a).unwrap())
        .collect::<Vec<Group>>();

    group_vector.push(Group {
        data: vec![Unit::ArrayUnit(vec![Unit::IntUnit(2)])],
    });

    group_vector.push(Group {
        data: vec![Unit::ArrayUnit(vec![Unit::IntUnit(6)])],
    });

    group_vector.sort_by(|a, b| match are_groups_ordered(a, b) {
        Some(true) => std::cmp::Ordering::Less,
        Some(false) => std::cmp::Ordering::Greater,
        None => std::cmp::Ordering::Equal,
    });

    let part2 = group_vector
        .iter()
        .enumerate()
        .filter(|(_, g)| {
            *g == &Group {
                data: vec![Unit::ArrayUnit(vec![Unit::IntUnit(2)])],
            } || *g
                == &Group {
                    data: vec![Unit::ArrayUnit(vec![Unit::IntUnit(6)])],
                }
        })
        .map(|(index, a)| {
            println!("{:?}", a);
            index + 1
        })
        .product::<usize>();

    println!("Part 2: {}", part2);
}

fn check_array_unit_ordered(part_1: &[Unit], part_2: &[Unit]) -> Option<bool> {
    let result = part_1
        .iter()
        .zip(part_2.iter())
        .map(|(a, b)| is_ordered(a, b))
        .find(|x| x.is_some());

    match result {
        Some(res) => res,
        None => {
            if part_1.len() == part_2.len() {
                None
            } else {
                Some(part_1.len() < part_2.len())
            }
        }
    }
}

fn are_groups_ordered(part_1: &Group, part_2: &Group) -> Option<bool> {
    check_array_unit_ordered(&part_1.data, &part_2.data)
}

fn is_ordered(part_1: &Unit, part_2: &Unit) -> Option<bool> {
    let result = match (part_1, part_2) {
        (Unit::ArrayUnit(arr_1), Unit::ArrayUnit(arr_2)) => check_array_unit_ordered(arr_1, arr_2),
        (Unit::ArrayUnit(_arr_1), Unit::IntUnit(int_2)) => {
            let converted_2 = Unit::ArrayUnit(vec![Unit::IntUnit(*int_2)]);
            is_ordered(part_1, &converted_2)
        }
        (Unit::IntUnit(int_1), Unit::ArrayUnit(_arr_2)) => {
            let converted_1 = Unit::ArrayUnit(vec![Unit::IntUnit(*int_1)]);
            is_ordered(&converted_1, part_2)
        }
        (Unit::IntUnit(int_1), Unit::IntUnit(int_2)) => {
            if int_1 == int_2 {
                None
            } else {
                Some(int_1 < int_2)
            }
        }
    };

    // println!("Comparing {:?} and {:?} => {:?}", part_1, part_2, result);

    result
}
