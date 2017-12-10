use regex::Regex;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::i64::MIN;

pub fn run(input: Vec<String>) {
    let part1 = solve(&input).0;
    println!("part 1: {}", part1);
    assert_eq!(part1, 3745);

    let part2 = solve(&input).1;
    println!("part 2: {}", part2);
    assert_eq!(part2, 4644);
}

fn solve(input: &Vec<String>) -> (i64, i64) {
    // Use a map to store the registers.
    let mut registers: HashMap<&str, i64> = HashMap::new();

    // Regular expression to parse each line of input.
    let re = Regex::new(r"(.+?) (inc|dec) ([-0-9]+) if (.+?) (.+?) ([-0-9]+)$").unwrap();
    let mut max = MIN;
    for line in input.iter() {
        match re.captures(line) {
            Some(cap) => {
                let cond_left = cap.get(4).unwrap().as_str();
                let cond_op = cap.get(5).unwrap().as_str();
                let cond_right: i64 = cap.get(6).unwrap().as_str().parse().unwrap();
                let target = cap.get(1).unwrap().as_str();
                let op = cap.get(2).unwrap().as_str();
                let val: i64 = cap.get(3).unwrap().as_str().parse().unwrap();

                // Store a function pointer in f. It will either increment or decrement.
                let f = match op {
                    "inc" => i64::add,
                    "dec" => i64::sub,
                    _ => panic!("invalid op: {}", line),
                };

                // Similar approach for comparing the register value with the constant.
                let cond = match cond_op {
                    "<" => i64::lt,
                    ">" => i64::gt,
                    ">=" => i64::ge,
                    "<=" => i64::le,
                    "==" => i64::eq,
                    "!=" => i64::ne,
                    _ => panic!("invalid cond: {}", line),
                };
                if cond(registers.entry(&cond_left).or_insert(0), &cond_right) {
                    let t = registers.entry(&target).or_insert(0);
                    *t = f(*t, val);
                    // Check if we need to update max.
                    if *t > max {
                        max = *t;
                    }
                }
            }
            None => panic!("invalid input: {}", line),
        }
    }

    return (*registers.values().max().unwrap(), max);
}
