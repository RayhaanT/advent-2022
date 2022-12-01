use crate::Solution;

fn result(a: u64, b: u64) -> u64 {
    if a == b {
        1
    } else if (a + 1) % 3 == b {
        2
    } else {
        0
    }
}

pub fn solve(input: String) -> Solution {
    let pairs = input.trim_end().split("\n").map(|l| {
        (
            match l.chars().nth(0).unwrap() {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => panic!("Bad char"),
            },
            match l.chars().nth(2).unwrap() {
                'X' => 0,
                'Y' => 1,
                'Z' => 2,
                _ => panic!("Bad char"),
            },
        )
    });

    let sum: u64 = pairs
        .clone()
        .fold(0, |acc, pair| acc + 1 + pair.1 + 3 * result(pair.0, pair.1));

    let sum2 = pairs.fold(0, |acc, pair| {
        acc + (match pair.1 {
            0 => 0 + 1 + ((pair.0 + 2) % 3),
            1 => 3 + 1 + pair.0,
            2 => 6 + 1 + ((pair.0 + 1) % 3),
            _ => panic!("Should not be possible"),
        })
    });

    Solution {
        first: sum.to_string(),
        second: sum2.to_string(),
    }
}
