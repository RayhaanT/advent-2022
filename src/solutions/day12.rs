use crate::Solution;
use std::collections::HashSet;
use std::collections::HashMap;

fn shortest_path(start: (usize, usize), grid: &Vec<Vec<char>>) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut open: HashMap<(usize, usize), u32> = HashMap::new();
    let mut closed: HashSet<(usize, usize)> = HashSet::new();
    open.insert(start, 0);

    let mut current = start;
    let mut res = u32::MAX;

    while !open.is_empty() {
        let mut cost = u32::MAX;
        for o in open.iter() {
            if *o.1 < cost {
                cost = *o.1;
                current = *o.0;
            }
        }

        if grid[current.0][current.1] == 'E' {
            res = cost;
            break;
        }

        for dir in 0..4 {
            let neighbour = match dir {
                0 => (current.0 + 1, current.1),
                1 => if current.0 > 0 { (current.0 - 1, current.1) } else { current },
                2 => (current.0, current.1 + 1),
                3 => if current.1 > 0 { (current.0, current.1 - 1) } else { current },
                _ => panic!("Bad direction")
            };

            if neighbour == current {
                continue;
            }

            let curr_elevation = if grid[current.0][current.1] == 'S' { 'a' as u32 } else { grid[current.0][current.1] as u32 };

            if neighbour.0 < height && neighbour.1 < width  {
                if closed.contains(&neighbour) {
                    continue;
                }
                let elevation = if grid[neighbour.0][neighbour.1] == 'E' { 'z' as u32 } else { grid[neighbour.0][neighbour.1] as u32 };
                if curr_elevation + 1 < elevation {
                    continue;
                }
                open.entry(neighbour).or_insert(cost + 1);
            }
        }

        open.remove(&current);
        closed.insert(current);
    }

    res
}

pub fn solve(input: String) -> Solution {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start: (usize, usize) = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (y, x);
            }
        }
    }

    let start_cost = shortest_path(start, &grid);

    let mut best_cost = start_cost;
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'a' {
                // let res = shortest_path((y, x), &grid);
                // println!("({} {}) = {}", x, y, res);
                best_cost = best_cost.min(shortest_path((y, x), &grid))
            }
        }
    }

    Solution {
        first: start_cost.to_string(),
        second: best_cost.to_string(),
    }
}
