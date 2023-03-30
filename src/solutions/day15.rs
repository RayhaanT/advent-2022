use crate::Solution;
use std::collections::HashSet;

struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    distance: i32,
}

impl Sensor {
    fn new(pos: (i32, i32), beacon: (i32, i32)) -> Self {
        Self {
            pos,
            beacon,
            distance: dist(pos, beacon),
        }
    }
}

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn solve(input: String) -> Solution {
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|l| {
            let sp: Vec<&str> = l.split(":").collect();
            let sxy: Vec<&str> = sp[0].split(",").collect();
            let bxy: Vec<&str> = sp[1].split(",").collect();

            Sensor::new(
                (
                    sxy[0].split("=").nth(1).unwrap().parse().unwrap(),
                    sxy[1].split("=").nth(1).unwrap().parse().unwrap(),
                ),
                (
                    bxy[0].split("=").nth(1).unwrap().parse().unwrap(),
                    bxy[1].split("=").nth(1).unwrap().parse().unwrap(),
                ),
            )
        })
        .collect();

    for s in &sensors {
        println!(
            "sensor: ({}, {}) | beacon: ({}, {}) | distance: {}",
            s.pos.0, s.pos.1, s.beacon.0, s.beacon.1, s.distance
        );
    }

    let mut beacons: HashSet<(i32, i32)> = HashSet::new();

    let mut leftmost = i32::MAX;
    let mut rightmost = i32::MIN;

    let mut count = 0;
    let target_y = 2000000;

    for s in &sensors {
        beacons.insert(s.beacon);
        leftmost = leftmost.min(s.pos.0 - s.distance);
        rightmost = rightmost.max(s.pos.0 + s.distance);
    }

    for x in leftmost..=rightmost {
        for s in &sensors {
            if dist(s.pos, (x, target_y)) <= s.distance {
                count += 1;
                break;
            }
        }
    }

    count -= beacons.iter().filter(|b| b.1 == target_y).count();

    Solution {
        first: count.to_string(),
        second: String::from("Incomplete"),
    }
}
