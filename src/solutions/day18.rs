use crate::Solution;
use std::{collections::HashSet, ops::Add};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Pos {
        Pos { x, y, z }
    }
}

impl Add for &Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

fn neighbours(pos: Pos) -> Vec<Pos> {
    vec![
        &pos + &Pos::new(0, 0, 1),
        &pos + &Pos::new(0, 1, 0),
        &pos + &Pos::new(1, 0, 0),
        &pos + &Pos::new(-1, 0, 0),
        &pos + &Pos::new(0, -1, 0),
        &pos + &Pos::new(0, 0, -1),
    ]
}

pub fn solve(input: String) -> Solution {
    let cubes: Vec<Pos> = input
        .lines()
        .map(|s| {
            let sp: Vec<i32> = s.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
            Pos::new(sp[0], sp[1], sp[2])
        })
        .collect();

    // set of cubes in the fluid
    let mut fluid: HashSet<Pos> = HashSet::new();
    // set of cubes on the surface of the fluid boundary
    let mut boundary: HashSet<Pos> = HashSet::new();

    let mut corner = Pos::new(0, 0, 0);
    let mut sides = 0;
    for cube in cubes.iter() {
        sides += 6;
        for n in neighbours(*cube) {
            if fluid.contains(&n) {
                sides -= 2;
            } else {
                boundary.insert(n);
            }
        }
        boundary.remove(&cube);
        fluid.insert(*cube);

        corner = Pos::new(
            cube.x.max(corner.x),
            cube.y.max(corner.y),
            cube.z.max(corner.z),
        );
    }

    // Start at some point outside the fluid and traverse a bounding volume
    corner = &corner + &Pos::new(1, 1, 1);
    let mut surface_area = 0;
    let mut open_set: HashSet<Pos> = HashSet::new();
    open_set.insert(corner);
    let mut closed_set: HashSet<Pos> = HashSet::new();

    while !open_set.is_empty() {
        let curr = *open_set.iter().next().unwrap();

        for n in neighbours(curr) {
            if fluid.contains(&n) {
                surface_area += 1;
            } else if !closed_set.contains(&n)
                && n.x >= -1
                && n.y >= -1
                && n.z >= -1
                && n.x <= corner.x
                && n.y <= corner.y
                && n.z <= corner.z
            {
                open_set.insert(n);
            }
        }
        open_set.remove(&curr);
        closed_set.insert(curr);
    }

    Solution {
        first: sides.to_string(),
        second: surface_area.to_string(),
    }
}
