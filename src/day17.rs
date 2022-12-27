use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Hash, Eq, PartialEq, Clone)]
struct Offset(usize, usize);

#[derive(Clone)]
struct BlockUnit {
    mask: HashSet<Offset>,
}

enum BlockType {
    HorizontalBar,
    Cross,
    L,
    VerticalBar,
    Cube,
}

impl BlockUnit {
    fn new(block_type: BlockType) -> Self {
        let mut h = HashSet::new();
        match block_type {
            BlockType::HorizontalBar => {
                h.insert(Offset(0, 0));
                h.insert(Offset(1, 0));
                h.insert(Offset(2, 0));
                h.insert(Offset(3, 0));
            }
            BlockType::Cross => {
                h.insert(Offset(0, 1));
                h.insert(Offset(2, 1));
                h.insert(Offset(1, 0));
                h.insert(Offset(1, 1));
                h.insert(Offset(1, 2));
            }
            BlockType::L => {
                h.insert(Offset(0, 2));
                h.insert(Offset(1, 2));
                h.insert(Offset(2, 0));
                h.insert(Offset(2, 1));
                h.insert(Offset(2, 2));
            }
            BlockType::VerticalBar => {
                h.insert(Offset(0, 0));
                h.insert(Offset(0, 1));
                h.insert(Offset(0, 2));
                h.insert(Offset(0, 3));
            }
            BlockType::Cube => {
                h.insert(Offset(0, 0));
                h.insert(Offset(0, 1));
                h.insert(Offset(1, 0));
                h.insert(Offset(1, 1));
            }
        }

        BlockUnit { mask: h }
    }

    fn init_start(&mut self, x: usize, y: usize) {
        todo!()
    }

    /// returns false if the movement did not occur
    fn jet_push(&mut self, direction: &char) -> bool {
        match direction {
            '>' => {}
            '<' => {}
            _ => panic!("Parsing error"),
        };
        todo!();
    }

    /// returns false if the movement did not occur
    fn fall_down(&mut self) -> bool {
        todo!();
    }
}

fn main() {
    let file = File::open("./data/day17.test").expect("file not found!");
    let movements_vec = io::BufReader::new(file)
        .lines()
        .into_iter()
        .next()
        .unwrap()
        .unwrap()
        .chars().collect::<Vec<char>>();
    let mut movements = movements_vec.iter();


    let h_bar = BlockUnit::new(BlockType::HorizontalBar);
    let v_bar = BlockUnit::new(BlockType::VerticalBar);
    let l_block = BlockUnit::new(BlockType::L);
    let cross = BlockUnit::new(BlockType::Cross);
    let cube = BlockUnit::new(BlockType::Cube);

    let current_height = [0_usize; 7];

    // loop till movements is empty
    'outer_loop: loop {
        for block in vec![
            h_bar.clone(),
            v_bar.clone(),
            l_block.clone(),
            cross.clone(),
            cube.clone(),
        ]
        .iter_mut()
        {
            block.init_start(2, current_height.iter().max().unwrap() + 3);

            // loop till block is stuck
            'block_dropping: loop {
                if let Some(direction) = movements.next() {
                    // jet push (take movement and figure out new position)
                    block.jet_push(direction);

                    // fall 1 unit down
                    let did_fall = block.fall_down();

                    if !did_fall {
                        break 'block_dropping;
                    }
                    else {
                        // update the height vector
                        todo!();
                    };
                } else {
                    break 'outer_loop;
                }
            }
        }
    }
}
