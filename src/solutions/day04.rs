use crate::Solution;

fn parse_pair(p: &str) -> (u32, u32) {
    let dash = p.find('-').unwrap();
    (
        p[0..dash].parse::<u32>().unwrap(),
        p[dash + 1..p.len()].parse::<u32>().unwrap(),
    )
}

fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)
}

fn overlaps_partial(a: (u32, u32), b: (u32, u32)) -> bool {
    (a.1 <= b.1 && a.1 >= b.0) || (b.1 <= a.1 && b.1 >= a.0)
}

pub fn solve(input: String) -> Solution {
    let pairs: Vec<((u32, u32), (u32, u32))> = input
        .lines()
        .map(|l| {
            let comma_pos = l.find(',').unwrap();
            (
                parse_pair(&l[0..comma_pos]),
                parse_pair(&l[comma_pos + 1..l.len()]),
            )
        })
        .collect();

    Solution {
        first: pairs
            .iter()
            .filter(|p| overlaps(p.0, p.1))
            .count()
            .to_string(),
        second: pairs
            .iter()
            .filter(|p| overlaps_partial(p.0, p.1))
            .count()
            .to_string(),
    }
}
