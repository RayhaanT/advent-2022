use crate::Solution;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Clone, Hash)]
struct Valve {
    name: String,
    flow: u32,
    neighbours: Vec<String>,
}

fn shortest_path(start: &Valve, target: &Valve, valves: &HashMap<String, Valve>) -> u32 {
    let mut closed: HashSet<&Valve> = HashSet::new();
    let mut open: HashMap<&Valve, u32> = HashMap::new();
    open.insert(start, 0);

    while !open.is_empty() {
        let lowest = open.iter().fold((start, u32::max_value()), |prev, curr| {
            if *curr.1 < prev.1 {
                (*curr.0, *curr.1)
            } else {
                prev
            }
        });

        open.remove(lowest.0);

        if lowest.0 == target {
            return lowest.1;
        }

        for n in lowest.0.neighbours.iter() {
            let neighbour = valves.get(n).unwrap();
            if !closed.contains(neighbour) {
                open.insert(neighbour, lowest.1 + 1);
            }
        }

        closed.insert(lowest.0);
    }

    u32::max_value()
}

fn pressure(
    start: usize,
    valves: &Vec<Vec<u32>>,
    rates: &Vec<u32>,
    open: &mut Vec<bool>,
    time: u32,
) -> (u32, Vec<usize>) {
    if time <= 1 {
        return (0, Vec::new());
    }

    let mut best = 0;
    let mut tail = Vec::new();

    for i in 1..valves.len() {
        if start == i || open[i] || valves[start][i] + 1 >= time {
            continue;
        }

        open[i] = true;
        let new_time = time - valves[start][i] - 1;
        let res = pressure(i, valves, rates, open, new_time);
        if res.0 > best {
            best = res.0;
            tail = res.1;
        }
        open[i] = false;
    }

    tail.push(start);
    (best + rates[start] * time, tail)
}

pub fn solve(input: String) -> Solution {
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|line| {
            let name = &line[6..8];
            let mut sp = line.split(";");
            let rate = sp.next().unwrap()[23..].parse::<u32>().unwrap();

            (
                name.to_string(),
                Valve {
                    name: name.to_string(),
                    flow: rate,
                    neighbours: sp.next().unwrap()[24..]
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect(),
                },
            )
        })
        .collect();

    let mut useful: Vec<&Valve> = vec![valves.get("AA").unwrap()];
    let mut rates: Vec<u32> = vec![0];
    for (_, valve) in valves.iter() {
        if valve.flow > 0 {
            useful.push(valve);
            rates.push(valve.flow);
        }
    }

    // Find shortest path between each useful valve to reduce the graph complexity
    let mut paths: Vec<Vec<u32>> = vec![vec![0; useful.len()]; useful.len()];
    for (vi, v) in useful.iter().enumerate() {
        for (wi, w) in useful.iter().enumerate() {
            paths[vi][wi] = shortest_path(v, w, &valves)
        }
    }

    for v in &useful {
        print!("{} ", v.name);
    }
    println!();

    for row in &paths {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }

    let mut open = vec![false; useful.len()];
    let first = pressure(0, &paths, &rates, &mut open, 30);
    let second = pressure(0, &paths, &rates, &mut open, 26);
    for v in second.1 {
        open[v] = true;
    }
    let res = pressure(0, &paths, &rates, &mut open, 26).0 + second.0;

    Solution {
        first: first.0.to_string(),
        second: res.to_string(),
    }
}
