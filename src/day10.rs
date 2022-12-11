use std::{
    fs::File,
    io::{self, BufRead},
    result,
};

enum Operation {
    Add(i32),
    Noop,
}

fn main() {
    let file = File::open("./data/day10.input").expect("file not found!");
    let operations = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
        .iter()
        .map(|s| {
            let mut input = s.split(' ');
            input.next();

            if let Some(amount) = input.next() {
                return Operation::Add(amount.parse().unwrap());
            };

            Operation::Noop
        })
        .collect::<Vec<Operation>>();

    let mut cycle: u32 = 1;
    let mut register: i32 = 1;
    let mut result_cycles: i32 = 0;
    let mut screen: Vec<&str> = vec![];

    let mut update_final_result = |cycle: u32, register: i32| {
        if (cycle == 20 || (cycle as i32 - 20) as i32 % 40 == 0) && cycle < 221 {
            result_cycles += cycle as i32 * register;
        };
    };

    let mut draw_on_screen = |register: i32, cycle: u32| {
        let start = (cycle - (cycle % 40))/ 40;
        let position = (start as i32 * 40) + register;
        match cycle - 1 {
            a if a >= position as u32 - 1 && a <= position as u32 + 1 => screen.push("#"),
            _ => screen.push("."),
        }
    };

    operations.iter().for_each(|operation| {
        draw_on_screen(register, cycle);
        // Cycle is running
        if let Operation::Add(amount) = operation {
            // Cycle stops
            cycle += 1;
            // Cycle is running
            draw_on_screen(register, cycle);
            update_final_result(cycle, register);

            // Add during the second cycle
            register += amount;
        }

        // Cycle stops
        cycle += 1;

        // Cycle is running
        update_final_result(cycle, register);
    });

    println!("Result: {}", result_cycles);

    screen.chunks(40).for_each(|chunk| {
        println!("{}", chunk.join(""));
    });
}
