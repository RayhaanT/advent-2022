use crate::Solution;

pub fn solve(input: String) -> Solution {
    let num_crates = 9;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_crates {
        stacks.push(Vec::new());
    }
    let lines: Vec<&str> = input.lines().collect();
    let mut num_initial = 0;
    for l in &lines {
        if l.chars().nth(0).unwrap() == ' ' {
            break;
        }
        num_initial += 1;
    }
    let initial: Vec<&str> = lines[0..num_initial].to_vec();
    let mut instrs: Vec<&str> = lines[num_initial + 2..lines.len()].to_vec();

    for i in 0..num_crates {
        for l in &initial {
            let c = l.chars().nth(4 * i + 1).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    for s in &mut stacks {
        s.reverse();
    }

    let mut stacks_two = stacks.clone();
    for l in &mut instrs {
        let split: Vec<&str> = l.split(' ').collect();
        let num = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        for i in 0..num {
            let popped = stacks[from].pop().unwrap();
            stacks[to].push(popped);
            let moved = stacks_two[from][stacks_two[from].len() - num + i];
            stacks_two[to].push(moved);
        }
        for _ in 0..num {
            stacks_two[from].pop();
        }
    }

    Solution {
        first: stacks.iter().map(|v| v.last().unwrap()).collect(),
        second: stacks_two.iter().map(|v| v.last().unwrap()).collect(),
    }
}
