use crate::Solution;
use std::collections::HashMap;

pub fn solve(input: String) -> Solution {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (y, x);
            }
            if *cell == 'E' {
                end = (y, x);
            }
        }
    }

    let height = grid.len();
    let width = grid[0].len();
    let mut best_dists: Vec<Vec<u32>> = vec![vec![u32::MAX; width]; height];
    best_dists[end.0][end.1] = 0;

    let mut open: HashMap<(usize, usize), u32> = HashMap::new();
    open.insert(end, 0);

    let mut current = start;

    while !open.is_empty() {
        let mut cost = u32::MAX;
        for o in open.iter() {
            if *o.1 < cost {
                cost = *o.1;
                current = *o.0;
            }
        }
        best_dists[current.0][current.1] = cost;

        for dir in 0..4 {
            let neighbour = match dir {
                0 => (current.0 + 1, current.1),
                1 => {
                    if current.0 > 0 {
                        (current.0 - 1, current.1)
                    } else {
                        current
                    }
                }
                2 => (current.0, current.1 + 1),
                3 => {
                    if current.1 > 0 {
                        (current.0, current.1 - 1)
                    } else {
                        current
                    }
                }
                _ => panic!("Bad direction"),
            };

            if neighbour == current {
                continue;
            }

            let curr_elevation = if grid[current.0][current.1] == 'E' {
                'z' as u32
            } else {
                grid[current.0][current.1] as u32
            };

            if neighbour.0 < height && neighbour.1 < width {
                if best_dists[neighbour.0][neighbour.1] != u32::MAX {
                    continue;
                }

                let elevation = if grid[neighbour.0][neighbour.1] == 'S' {
                    'a' as u32
                } else {
                    grid[neighbour.0][neighbour.1] as u32
                };

                if curr_elevation > elevation + 1 {
                    continue;
                }
                open.entry(neighbour).or_insert(cost + 1);
            }
        }

        open.remove(&current);
    }

    let best_cost = best_dists
        .iter()
        .enumerate()
        .fold(u32::MAX, |res, (y, next)| {
            let row_best = next.iter().enumerate().fold(u32::MAX, |res, (x, next)| {
                if grid[y][x] == 'a' {
                    res.min(*next)
                } else {
                    res
                }
            });
            res.min(row_best)
        });

    Solution {
        first: best_dists[start.0][start.1].to_string(),
        second: best_cost.to_string(),
    }
}
