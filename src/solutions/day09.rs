use crate::Solution;
use std::collections::HashSet;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn touching(a: (i32, i32), b: (i32, i32)) -> bool {
    (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1
}

pub fn solve(input: String) -> Solution {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut end_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];

    // Add initial positions to set
    visited.insert(knots[1]);
    end_visited.insert(*knots.last().unwrap());

    let instrs: Vec<(Direction, u32)> = input
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.split(' ').collect();
            (
                match s[0] {
                    "U" => Direction::UP,
                    "D" => Direction::DOWN,
                    "R" => Direction::LEFT,
                    "L" => Direction::RIGHT,
                    _ => panic!("Invalid direction"),
                },
                s[1].parse::<u32>().unwrap(),
            )
        })
        .collect();

    for i in instrs {
        for _ in 0..i.1 {
            let h = knots[0];
            knots[0] = match i.0 {
                Direction::UP => (h.0, h.1 + 1),
                Direction::DOWN => (h.0, h.1 - 1),
                Direction::RIGHT => (h.0 + 1, h.1),
                Direction::LEFT => (h.0 - 1, h.1),
            };

            for k in 1..knots.len() {
                if !touching(knots[k - 1], knots[k]) {
                    let h = knots[k - 1];
                    // Move 1 step in the x direction
                    if h.0 != knots[k].0 {
                        knots[k].0 = knots[k].0 + if h.0 > knots[k].0 { 1 } else { -1 };
                    }
                    // Move 1 step in the y direction
                    if h.1 != knots[k].1 {
                        knots[k].1 = knots[k].1 + if h.1 > knots[k].1 { 1 } else { -1 };
                    }
                }
            }
            visited.insert(knots[1]);
            end_visited.insert(*knots.last().unwrap());
        }
    }

    Solution {
        first: visited.iter().count().to_string(),
        second: end_visited.iter().count().to_string(),
    }
}
