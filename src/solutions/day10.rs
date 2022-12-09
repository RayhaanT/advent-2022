use crate::Solution;

fn on_tick(cycle: usize, x: i32, res: &mut i32, crt: &mut Vec<Vec<char>>) {
    if cycle % 20 == 0 && cycle % 40 != 0 {
        *res += x * cycle as i32;
    }

    let cind = cycle - 1;
    if (x - (cind % 40) as i32).abs() <= 1 {
        crt[cind / 40][cind % 40] = '#';
    }
}

pub fn solve(input: String) -> Solution {
    let mut x = 1;
    let mut res = 0;
    let mut cycle = 1;
    let mut crt: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];

    on_tick(cycle, x, &mut res, &mut crt);
    for instr in input.lines() {
        if instr == "noop" {
            cycle += 1;
        } else {
            cycle += 1;
            on_tick(cycle, x, &mut res, &mut crt);
            x += instr.split(' ').last().unwrap().parse::<i32>().unwrap();
            cycle += 1;
        }
        on_tick(cycle, x, &mut res, &mut crt);
    }

    Solution {
        first: res.to_string(),
        second: crt.iter().fold(String::from(""), |acc, line| {
            format!("{}\n{}", acc, line.iter().collect::<String>())
        }),
    }
}
