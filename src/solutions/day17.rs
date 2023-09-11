use std::{collections::HashSet, u8};

use crate::Solution;

type Block = Vec<(usize, usize)>;

fn collides(column: &Vec<u8>, block: &Block, x: usize, y: usize) -> bool {
    if x >= 7 {
        return true;
    }

    for pair in block.iter() {
        let col_x = x + pair.0;
        let col_y = y + pair.1;

        if col_x >= 7 {
            return true;
        }

        if col_y >= column.len() {
            continue;
        }

        let row = column[col_y];
        if row & (1 << col_x) != 0 {
            return true;
        }
    }
    return false;
}

pub fn solve(input: String) -> Solution {
    let mut column: Vec<u8> = Vec::new();

    let flat: Block = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let plus: Block = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
    let corner: Block = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    let tall: Block = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let square: Block = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
    let blocks: Vec<Block> = vec![flat, plus, corner, tall, square];

    let mut block_ind = 0;
    let mut block = &blocks[block_ind];

    let mut x = 2;
    let mut y = 3;

    let pattern = input.lines().next().unwrap().as_bytes();
    let mut pattern_ind = 0;

    while block_ind < 2022 {
        let c = pattern[pattern_ind] as char;
        pattern_ind = (pattern_ind + 1) % pattern.len();

        // Blow
        if c == '>' && !collides(&column, &block, x + 1, y) {
            x += 1;
        } else if c == '<' && x > 0 && !collides(&column, &block, x - 1, y) {
            x -= 1;
        }

        // Drop
        if y == 0 || collides(&column, &block, x, y - 1) {
            for pair in block.iter() {
                let col_x = x + pair.0;
                let col_y = y + pair.1;

                while col_y >= column.len() {
                    column.push(0);
                }

                column[col_y] |= 1 << col_x;
            }

            block_ind += 1;
            block = &blocks[block_ind % blocks.len()];
            x = 2;
            y = column.len() + 3;

            // for row in column.iter().rev() {
            //     for i in 0..7 {
            //         if row & (1 << i) != 0 {
            //             print!("#");
            //         } else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }
            // println!();
        } else {
            y -= 1;
        }
    }

    Solution {
        first: column.len().to_string(),
        second: String::from("Incomplete"),
    }
}
