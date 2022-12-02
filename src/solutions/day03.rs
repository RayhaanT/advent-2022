use crate::Solution;

fn priority(c: char) -> u64 {
    if c as u8 >= 'a' as u8 {
        (c as u8 + 1 - 'a' as u8) as u64
    } else {
        (c as u8 + 27 - 'A' as u8) as u64
    }
}

pub fn solve(input: String) -> Solution {
    let sacks: Vec<&str> = input.trim_end().split("\n").collect();

    let compartments: Vec<(&str, &str)> = sacks
        .iter()
        .map(|line| {
            let size = line.len();
            (&line[0..size / 2], &line[size / 2..size])
        })
        .collect();

    let mut sum = 0;
    for s in &compartments {
        for c in s.0.chars() {
            if s.1.contains(c) {
                sum += priority(c);
                break;
            }
        }
    }

    let mut sum2 = 0;
    for s in 0..sacks.len() / 3 {
        let first = sacks[s * 3];
        let second = sacks[s * 3 + 1];
        let third = sacks[s * 3 + 2];
        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                sum2 += priority(c);
                break;
            }
        }
    }

    Solution {
        first: sum.to_string(),
        second: sum2.to_string(),
    }
}
