use common::sfy::Sfy;
use regex::Regex;

pub fn run(input: &[u8]) {
    println!("pro-tip: compile with --release!");
    let part1 = solve_part1(input).sfy();
    println!("part 1: {}", part1);
    assert_eq!(part1, "bkgcdefiholnpmja");

    let part2 = solve_part2(input, 1000000000).sfy();
    println!("part 2: {}", part2);
    assert_eq!(part2, "knmdfoijcbpghlea");
}

fn solve_part1(input: &[u8]) -> Vec<u8> {
    let a = "abcdefghijklmnop";
    return transform(input.sfy(), a.as_bytes());
}

fn solve_part2(input: &[u8], iter: usize) -> Vec<u8> {
    let a = "abcdefghijklmnop".as_bytes();

    // measure cycle
    let mut b: Vec<u8> = a.to_vec();
    let mut cycle = 0;
    loop {
        b = transform(input.sfy(), &b);
        cycle += 1;
        if a.to_vec() == b {
            break;
        }
    }
    println!("cycle length: {}", cycle);

    // perform iter modulo cycle transformations
    let mut b = a.to_vec();
    for _ in 0..(iter % cycle) {
        b = transform(input.sfy(), &b);
    }
    return b;
}

#[allow(non_snake_case)]
fn transform(moves: String, state: &[u8]) -> Vec<u8> {
    let L = state.len();
    let mut r = Vec::new();
    for i in 0..L {
        r.push(state[i]);
    }

    let re_s = Regex::new(r"s([0-9]+)").unwrap();
    let re_x = Regex::new(r"x([0-9]+)/([0-9]+)").unwrap();
    let re_p = Regex::new(r"p([a-z]+)/([a-z]+)").unwrap();

    for m in moves.split(",") {
        let cap = re_s.captures(m);
        if cap.is_some() {
            let spin: usize = cap.unwrap().get(1).unwrap().as_str().parse().unwrap();
            let t = r.clone();
            for i in 0..state.len() {
                r[(i + spin) % L] = t[i];
            }
            continue;
        }
        let cap = re_x.captures(m);
        if cap.is_some() {
            let cap = cap.unwrap();
            let a: usize = cap.get(1).unwrap().as_str().parse().unwrap();
            let b: usize = cap.get(2).unwrap().as_str().parse().unwrap();
            let t = r[a];
            r[a] = r[b];
            r[b] = t;
            continue;
        }
        let cap = re_p.captures(m);
        if cap.is_some() {
            let cap = cap.unwrap();
            let pa: &str = cap.get(1).unwrap().as_str();
            let pb: &str = cap.get(2).unwrap().as_str();
            let a = r.iter().position(|&c| c == pa.as_bytes()[0]).unwrap();
            let b = r.iter().position(|&c| c == pb.as_bytes()[0]).unwrap();

            let t = r[a];
            r[a] = r[b];
            r[b] = t;
            continue;
        }
        panic!("invalid input: {}", m);
    }

    return r;
}
