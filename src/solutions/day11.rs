use crate::Solution;

#[derive(Clone)]
enum Operation {
    PLUS,
    TIMES,
}

#[derive(Clone)]
struct Monkey {
    inventory: Vec<i64>,
    operation: Operation,
    constant: Option<i64>,
    test: i64,
    buddies: (usize, usize),
    inspected: u64,
}

fn get_monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    let mut max1 = 0;
    let mut max2 = 0;

    for monkey in monkeys {
        if monkey.inspected > max1 {
            max2 = max1;
            max1 = monkey.inspected;
        } else if monkey.inspected > max2 {
            max2 = monkey.inspected;
        }
    }

    max1 * max2
}

pub fn solve(input: String) -> Solution {
    let reference_monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<&str> = block.lines().collect();
            Monkey {
                inventory: lines[1][17..]
                    .split(",")
                    .map(|n| n.trim().parse::<i64>().unwrap())
                    .collect(),
                operation: if lines[2].contains('+') {
                    Operation::PLUS
                } else {
                    Operation::TIMES
                },
                constant: lines[2].split(" ").last().unwrap().parse::<i64>().ok(),
                test: lines[3].split(" ").last().unwrap().parse::<i64>().unwrap(),
                buddies: (
                    lines[4]
                        .split(" ")
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                    lines[5]
                        .split(" ")
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                ),
                inspected: 0,
            }
        })
        .collect();

    // Do the first part
    let mut monkeys = reference_monkeys.clone();
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].inventory.len() {
                let mut new_worry = monkeys[m].inventory[0];
                match monkeys[m].operation {
                    Operation::PLUS => match monkeys[m].constant {
                        None => new_worry += new_worry,
                        Some(n) => new_worry += n,
                    },
                    Operation::TIMES => match monkeys[m].constant {
                        None => new_worry *= new_worry,
                        Some(n) => new_worry *= n,
                    },
                }
                new_worry /= 3;
                monkeys[m].inventory.remove(0);
                let passing = match new_worry % monkeys[m].test {
                    0 => monkeys[m].buddies.0,
                    _ => monkeys[m].buddies.1,
                };
                monkeys[passing].inventory.push(new_worry);
                monkeys[m].inspected += 1;
            }
        }
    }

    let day1 = get_monkey_business(&monkeys);
    let common_mod = reference_monkeys.iter().fold(1, |acc, m| acc * m.test);

    // Do the second part with modular arithmetic stuff
    monkeys = reference_monkeys.clone();
    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            for _ in 0..monkeys[m].inventory.len() {
                let mut new_worry = monkeys[m].inventory[0];
                match monkeys[m].operation {
                    Operation::PLUS => match monkeys[m].constant {
                        None => new_worry += new_worry,
                        Some(n) => new_worry += n,
                    },
                    Operation::TIMES => match monkeys[m].constant {
                        None => new_worry *= new_worry,
                        Some(n) => new_worry *= n,
                    },
                }
                // Mod out by the product of all the tests
                new_worry = new_worry % common_mod;
                monkeys[m].inventory.remove(0);
                let passing = match new_worry % monkeys[m].test {
                    0 => monkeys[m].buddies.0,
                    _ => monkeys[m].buddies.1,
                };
                monkeys[passing].inventory.push(new_worry);
                monkeys[m].inspected += 1;
            }
        }
    }

    let day2 = get_monkey_business(&monkeys);

    Solution {
        first: day1.to_string(),
        second: day2.to_string(),
    }
}
