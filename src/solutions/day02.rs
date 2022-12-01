use crate::Solution;

fn result(a: u64, b: u64) -> u64 {
    if a == b {
        1
    } else if a + 1 == b || a + 1 == 4 && b == 1 {
        2
    } else {
        0
    }
}

pub fn solve(input: String) -> Solution {
    let pairs: Vec<(u64, u64)> = input
        .trim_end()
        .split("\n")
        .map(|l| {
            (
                match l.chars().nth(0).unwrap() {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => panic!("Bad char"),
                },
                match l.chars().nth(2).unwrap() {
                    'X' => 1,
                    'Y' => 2,
                    'Z' => 3,
                    _ => panic!("Bad char"),
                },
            )
        })
        .collect();

    let sum: u64 = pairs
        .clone()
        .iter()
        .fold(0, |acc, pair| acc + pair.1 + 3 * result(pair.0, pair.1));

    let sum2 = pairs.iter().fold(0, |acc, pair| {
        acc + (match pair.1 {
            1 => 0 + (if pair.0 == 1 { 3 } else { pair.0 - 1 }),
            2 => 3 + pair.0,
            3 => 6 + (if pair.0 == 3 { 1 } else { pair.0 + 1 }),
            _ => panic!("Should not be possible"),
        })
    });

    Solution {
        first: sum.to_string(),
        second: sum2.to_string(),
    }
}
