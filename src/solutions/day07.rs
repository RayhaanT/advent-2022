use crate::Solution;
use std::collections::HashMap;

struct File {
    path: String,
    parent: String,
    children: Vec<String>,
    size: u32,
}

fn getSize(filename: &String, filesystem: &HashMap<String, File>) -> u32 {
    let file = filesystem.get(filename).unwrap();
    file.children
        .iter()
        .map(|f| getSize(&filesystem.get(f).unwrap().path, filesystem))
        .sum::<u32>()
        + file.size
}

pub fn solve(input: String) -> Solution {
    let mut filesystem: HashMap<String, File> = HashMap::new();
    let commands: Vec<&str> = input.lines().collect();
    let mut line = 0;
    let mut currentDir: String = String::from("");

    while line < commands.len() {
        let decomp: Vec<&str> = commands[line].split(" ").collect();
        match decomp[1] {
            "cd" => {
                if decomp[2] == ".." {
                    currentDir = filesystem.get(&currentDir).unwrap().parent.clone();
                } else {
                    let oldDir = currentDir;
                    if oldDir == String::from("") {
                        currentDir = String::from("/");
                    } else if oldDir == String::from("/") {
                        currentDir = oldDir.clone() + decomp[2];
                    } else {
                        currentDir = oldDir.clone() + "/" + decomp[2];
                    }

                    filesystem.entry(currentDir.clone()).or_insert(File {
                        path: currentDir.clone(),
                        parent: String::from(oldDir),
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
                    let filename = if currentDir == String::from("/") {
                        String::from("/") + split[1]
                    } else {
                        currentDir.clone() + "/" + split[1]
                    };

                    if split[0] == "dir" {
                        filesystem.entry(filename.clone()).or_insert(File {
                            path: filename.clone(),
                            parent: currentDir.clone(),
                            children: Vec::new(),
                            size: 0,
                        });

                        filesystem
                            .entry(currentDir.clone())
                            .and_modify(|f| f.children.push(filename));
                    } else {
                        filesystem
                            .entry(currentDir.clone())
                            .and_modify(|f| f.size += split[0].parse::<u32>().unwrap());
                    }
                    line += 1;
                }
            }
            _ => panic!("Invalid command"),
        }
    }

    let mut sizes: Vec<u32> = filesystem.keys().map(|k| getSize(k, &filesystem)).collect();
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
