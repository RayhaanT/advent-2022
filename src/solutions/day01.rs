use crate::Solution;

pub fn solve(input: String) -> Solution {
    let mut inventories: Vec<i32> = input
        .trim_end()
        .split("\n\n")
        .map(|inv| {
            inv.split("\n")
                .fold(0, |acc, item| acc + item.parse::<i32>().unwrap())
        })
        .collect();

    inventories.sort_unstable_by(|a, b| b.cmp(a));

    Solution {
        first: inventories[0].to_string(),
        second: inventories[0..3]
            .iter()
            .fold(0, |acc, item| acc + item)
            .to_string(),
    }
}
