use crate::Solution;
use std::collections::HashSet;

fn drop_sand(field: &HashSet<(usize, usize)>, btm: usize) -> (usize, usize) {
    let mut sand_pos = (500, 0);
    while sand_pos.1 <= btm {
        let down = (sand_pos.0, sand_pos.1 + 1);
        let left = (sand_pos.0 - 1, sand_pos.1 + 1);
        let right = (sand_pos.0 + 1, sand_pos.1 + 1);

        if !field.contains(&down) {
            sand_pos = down;
        } else if !field.contains(&left) {
            sand_pos = left;
        } else if !field.contains(&right) {
            sand_pos = right;
        } else {
            break;
        }
    }
    sand_pos
}

pub fn solve(input: String) -> Solution {
    let mut field: HashSet<(usize, usize)> = HashSet::new();

    let mut xbounds = (usize::MAX, 0);
    let mut ybounds = (usize::MAX, 0);

    for l in input.lines() {
        let endpoints = l.split("->").map(|s| {
            s.trim()
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>()
        });

        let mut last = (0, 0);
        endpoints.enumerate().for_each(|(ind, val)| {
            let start = (val[0], val[1]);
            if ind != 0 {
                let startx = start.0.min(last.0);
                let starty = start.1.min(last.1);
                let endx = start.0 + last.0 - startx;
                let endy = start.1 + last.1 - starty;
                for i in startx..=endx {
                    for j in starty..=endy {
                        field.insert((i, j));
                    }
                }
                xbounds = (xbounds.0.min(startx), xbounds.1.max(endx));
                ybounds = (ybounds.0.min(starty), ybounds.1.max(endy));
            }
            last = start;
        });
    }

    let mut sand_dropped = 0;
    let mut sand;
    loop {
        sand = drop_sand(&field, ybounds.1);
        if sand.1 < ybounds.1 {
            sand_dropped += 1;
            field.insert(sand);
        } else {
            break;
        }
    }

    let first = sand_dropped;
    while sand.1 != 0 {
        sand_dropped += 1;
        sand = drop_sand(&field, ybounds.1);
        field.insert(sand);
    }

    // for y in ybounds.0..=ybounds.1 {
    //     for x in xbounds.0..=xbounds.1 {
    //         if !field.contains(&(x, y)) {
    //             print!(" ");
    //         } else {
    //             print!("#");
    //         }
    //     }
    //     println!("");
    // }

    Solution {
        first: first.to_string(),
        second: sand_dropped.to_string(),
    }
}
