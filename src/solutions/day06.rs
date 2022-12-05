use crate::Solution;

fn unique(s: &Vec<char>) -> bool {
    for i in 0..s.len() {
        if s[i + 1..s.len()].contains(&s[i]) {
            return false;
        }
    }
    true
}

pub fn solve(input: String) -> Solution {
    let mut start = 4;
    let mut slice: Vec<char> = input[0..4].chars().collect();

    while !unique(&slice) {
        slice.push(input.chars().nth(start).unwrap());
        slice.remove(0);
        start += 1;
    }
    let ans1 = start.to_string();

    start = 14;
    slice = input[0..14].chars().collect();
    while !unique(&slice) {
        slice.push(input.chars().nth(start).unwrap());
        slice.remove(0);
        start += 1;
    }

    Solution {
        first: ans1,
        second: start.to_string(),
    }
}
