use crate::Solution;

pub fn solve(input: String) -> Solution {
    let mut grid: Vec<Vec<(u32, bool)>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| (c.to_digit(10).unwrap(), false))
                .collect()
        })
        .collect();

    for dir in 0..4 {
        let bounds = if dir % 2 == 0 {
            (grid.len(), grid[0].len())
        } else {
            (grid[0].len(), grid.len())
        };
        for y in 0..bounds.0 {
            let mut tallest = 0;
            for x in 0..bounds.1 {
                let x_ind = if dir < 2 { x } else { bounds.1 - x - 1 };
                let cell: &mut (u32, bool) = if dir % 2 == 0 {
                    &mut grid[y][x_ind]
                } else {
                    &mut grid[x_ind][y]
                };
                if x == 0 || cell.0 > tallest {
                    tallest = cell.0;
                    cell.1 = true;
                }
            }
        }
    }

    let mut max = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let mut view = 1;

            for dir in 0..4 {
                let mut walked = 0;
                let bound = if dir % 2 == 0 {
                    grid[0].len()
                } else {
                    grid.len()
                } as i32;

                let step: i32 = if dir < 2 { 1 } else { -1 };
                let mut walk = step;
                let axis = if dir % 2 == 0 { x } else { y };

                while 0 <= walk + axis as i32 && (walk + axis as i32) < bound {
                    walked += 1;

                    if (if dir % 2 == 0 {
                        grid[y][(x as i32 + walk) as usize].0
                    } else {
                        grid[(y as i32 + walk) as usize][x].0
                    }) >= cell.0
                    {
                        break;
                    }

                    walk += step;
                }
                view *= walked;
            }

            if max < view {
                max = view;
            }
        }
    }

    Solution {
        first: grid
            .iter()
            .fold(0, |acc, line| acc + line.iter().filter(|c| c.1).count())
            .to_string(),
        second: max.to_string(),
    }
}
