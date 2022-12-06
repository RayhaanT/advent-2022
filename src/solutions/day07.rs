use crate::Solution;
use std::collections::HashMap;

struct File {
    path: String,
    parent: String,
    children: Vec<String>,
    size: u32,
}

fn get_size(
    filename: &String,
    filesystem: &HashMap<String, File>,
    memo: &mut HashMap<String, u32>,
) -> u32 {
    match memo.get(filename) {
        Some(i) => *i,
        None => {
            let file = filesystem.get(filename).unwrap();
            let res = file
                .children
                .iter()
                .map(|f| get_size(&filesystem.get(f).unwrap().path, filesystem, memo))
                .sum::<u32>()
                + file.size;
            memo.insert(filename.clone(), res);
            res
        }
    }
}

pub fn solve(input: String) -> Solution {
    let mut filesystem: HashMap<String, File> = HashMap::new();
    let commands: Vec<&str> = input.lines().collect();
    let mut line = 0;
    let mut current_dir: String = String::from("");

    while line < commands.len() {
        let decomp: Vec<&str> = commands[line].split(" ").collect();
        match decomp[1] {
            "cd" => {
                if decomp[2] == ".." {
                    current_dir = filesystem.get(&current_dir).unwrap().parent.clone();
                } else {
                    let old_dir = current_dir;
                    if old_dir == String::from("") {
                        current_dir = String::from("/");
                    } else if old_dir == String::from("/") {
                        current_dir = old_dir.clone() + decomp[2];
                    } else {
                        current_dir = old_dir.clone() + "/" + decomp[2];
                    }

                    filesystem.entry(current_dir.clone()).or_insert(File {
                        path: current_dir.clone(),
                        parent: String::from(old_dir),
                        children: Vec::new(),
                        size: 0,
                    });
                }
                line += 1;
            }
            "ls" => {
                line += 1;
                while line < commands.len() && commands[line].chars().next().unwrap() != '$' {
                    let split: Vec<&str> = commands[line].split(" ").collect();
                    let filename = if current_dir == String::from("/") {
                        String::from("/") + split[1]
                    } else {
                        current_dir.clone() + "/" + split[1]
                    };

                    if split[0] == "dir" {
                        filesystem.entry(filename.clone()).or_insert(File {
                            path: filename.clone(),
                            parent: current_dir.clone(),
                            children: Vec::new(),
                            size: 0,
                        });

                        filesystem
                            .entry(current_dir.clone())
                            .and_modify(|f| f.children.push(filename));
                    } else {
                        filesystem
                            .entry(current_dir.clone())
                            .and_modify(|f| f.size += split[0].parse::<u32>().unwrap());
                    }
                    line += 1;
                }
            }
            _ => panic!("Invalid command"),
        }
    }

    let mut memo: HashMap<String, u32> = HashMap::new();
    let mut sizes: Vec<u32> = filesystem
        .keys()
        .map(|k| get_size(k, &filesystem, &mut memo))
        .collect();
    sizes.sort();
    let root_size = sizes.last().unwrap();
    let max = 40000000;
    let mut min_diff: u32 = 0;
    for i in &sizes {
        if *i >= root_size - max {
            min_diff = *i;
            break;
        }
    }

    Solution {
        first: sizes
            .iter()
            .filter(|s| **s < 100000)
            .sum::<u32>()
            .to_string(),
        second: min_diff.to_string(),
    }
}
