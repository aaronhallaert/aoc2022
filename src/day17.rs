use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

const CAVE_WIDTH: usize = 7;

#[derive(Clone, Debug)]
struct Block {
    pub mask: Vec<(isize, isize)>,
}

enum BlockType {
    HorizontalBar,
    Cross,
    L,
    VerticalBar,
    Cube,
}

impl Block {
    fn new(block_type: BlockType) -> Self {
        match block_type {
            BlockType::HorizontalBar => Block {
                mask: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            },
            BlockType::Cross => Block {
                mask: vec![(0, 1), (2, 1), (1, 0), (1, 1), (1, 2)],
            },
            BlockType::L => Block {
                mask: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            },
            BlockType::VerticalBar => Block {
                mask: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            },
            BlockType::Cube => Block {
                mask: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            },
        }
    }

    fn init_start(&mut self, cave: &Cave) {
        let start_position = cave.calc_start_position();
        self.mask.iter_mut().for_each(|offset| {
            offset.0 += start_position.0;
            offset.1 += start_position.1;
        });
    }

    /// returns false if the movement did not occur
    fn jet_push(&mut self, direction: &char, cave: &Cave) -> bool {
        // check the spaces first
        let can_move: bool = match direction {
            '>' => self
                .mask
                .iter()
                .map(|(a, b)| ((a + 1), *b))
                .all(|space| cave.is_space_free(space)),
            '<' => self
                .mask
                .iter()
                .map(|(a, b)| ((a - 1), *b))
                .all(|space| cave.is_space_free(space)),
            _ => panic!("Parsing error"),
        };

        if can_move {
            // update the block
            match direction {
                '>' => {
                    self.mask.iter_mut().for_each(|space| {
                        space.0 += 1;
                    });
                }
                '<' => {
                    self.mask.iter_mut().for_each(|space| {
                        space.0 -= 1;
                    });
                }
                _ => panic!("Parsing error"),
            }
        }

        can_move
    }

    /// returns false if the movement did not occur
    fn fall_down(&mut self, cave: &Cave) -> bool {
        // check the spaces first
        let can_move: bool = self
            .mask
            .iter()
            .map(|(a, b)| (*a, b - 1))
            .all(|space| cave.is_space_free(space));

        if can_move {
            self.mask.iter_mut().for_each(|space| space.1 -= 1)
        }

        can_move
    }
}

struct Cave {
    occupied_spaces: HashSet<(isize, isize)>,
    dropped_blocks: usize,
}

impl Cave {
    fn new() -> Self {
        Self {
            occupied_spaces: HashSet::new(),
            dropped_blocks: 0,
        }
    }

    fn print(&self, block: Option<&Block>) {
        let max_height = self.occupied_spaces.iter().map(|s| s.1).max().unwrap_or(0) + 10;
        println!("---------------------------");
        println!("Max height: {}", max_height);
        println!("Falling block: {:?}", block);

        for y in (0..max_height + 1).rev() {
            for x in 0..7 {
                match self.occupied_spaces.contains(&(x, y)) {
                    true => {
                        print!("#")
                    }
                    false => {
                        if let Some(block) = block {
                            if block.mask.contains(&(x, y)) {
                                print!("@")
                            } else {
                                print!(".")
                            }
                        } else {
                            print!(".")
                        }
                    }
                }
            }
            println!()
        }
    }

    fn is_space_free(&self, space: (isize, isize)) -> bool {
        !self.occupied_spaces.contains(&space)
            && space.0 >= 0
            && space.0 < CAVE_WIDTH as isize
            && space.1 >= 0
    }

    fn calc_start_position(&self) -> (isize, isize) {
        (
            2,
            self.occupied_spaces.iter().map(|s| s.1).max().unwrap_or(-1) + 4,
        )
    }

    fn fix_block(&mut self, block: &Block) {
        block.mask.iter().for_each(|offset| {
            self.occupied_spaces.insert(*offset);
        });
        self.dropped_blocks += 1;
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
        .chars()
        .collect::<Vec<char>>();
    let mut movements = movements_vec.iter().cycle();

    let h_bar = Block::new(BlockType::HorizontalBar);
    let v_bar = Block::new(BlockType::VerticalBar);
    let l_block = Block::new(BlockType::L);
    let cross = Block::new(BlockType::Cross);
    let cube = Block::new(BlockType::Cube);

    let mut cave = Cave::new();

    // loop till movements is empty

    'outer_loop: loop {
        for block in vec![
            h_bar.clone(),
            cross.clone(),
            l_block.clone(),
            v_bar.clone(),
            cube.clone(),
        ]
        .iter_mut()
        {
            block.init_start(&cave);
            // cave.print(Some(block));

            // loop till block is stuck
            'block_dropping: loop {
                if let Some(direction) = movements.next() {
                    // jet push (take movement and figure out new position)
                    block.jet_push(direction, &cave);
                    // cave.print(Some(block));

                    let did_fall = block.fall_down(&cave);
                    // cave.print(Some(block));

                    if !did_fall {
                        // update the cave
                        cave.fix_block(block);
                        if cave.dropped_blocks == 2022 {
                            // cave.print(None);
                            break 'outer_loop;
                        };
                        break 'block_dropping;
                    };
                } else {
                    panic!("For some reason, cycle is broken");
                }
            }
        }
    }

    println!("Dropped blocks: {}", cave.dropped_blocks);

    println!(
        "Result: {}",
        cave.occupied_spaces.iter().map(|s| s.1).max().unwrap() + 1
    )
}
