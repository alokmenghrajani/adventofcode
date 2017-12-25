use regex::Regex;
use std::collections::HashMap;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(input.clone());
    println!("part 1: {}", part1);
    assert_eq!(part1, 4225);

    let part2 = solve_part2();
    println!("part 2: {}", part2);
    assert_eq!(part2, 905);
}

fn solve_part1(input: Vec<String>) -> i64 {
    // Parse the input
    let prog = parse(input);

    // Run the code
    let p = Process::new(prog);
    return p.run_part1();
}


fn solve_part2() -> i64 {
    // Solving part2 manually is much easier than writing an optimizer.
    let mut h = 0;
    for i in 0..1001 {
        let b = 106700 + 17 * i;
        // seems the code is doing a primality test
        for e in 2..b {
            if b % e == 0 {
                h += 1;
                break;
            }
        }
    }
    return h;
}

struct Process {
    pc: i64,
    reg: HashMap<char, i64>,
    prog: Vec<Op>,
}

impl Process {
    fn new(prog: Vec<Op>) -> Process {
        return Process {
            pc: 0,
            reg: HashMap::new(),
            prog: prog,
        };
    }

    fn run_part1(mut self) -> i64 {
        let mut mul_counter = 0;
        loop {
            if (self.pc < 0) || (self.pc as usize >= self.prog.len()) {
                // we are out of range
                println!("  debug: pc is out of bounds");
                break;
            }
            let op = self.prog[self.pc as usize];
            match op {
                Op::Sub(Param::R(p1), p2) => {
                    let t = get_val(&mut self.reg, Param::R(p1)) - get_val(&mut self.reg, p2);
                    self.reg.insert(p1, t);
                    self.pc += 1;
                }
                Op::Mul(Param::R(p1), p2) => {
                    let t = get_val(&mut self.reg, Param::R(p1)) * get_val(&mut self.reg, p2);
                    self.reg.insert(p1, t);
                    self.pc += 1;
                    mul_counter += 1;
                }
                Op::Set(Param::R(p1), p2) => {
                    let t = get_val(&mut self.reg, p2);
                    self.reg.insert(p1, t);
                    self.pc += 1;
                }
                Op::Jnz(p1, p2) => {
                    let p1 = get_val(&mut self.reg, p1);
                    let p2 = get_val(&mut self.reg, p2);
                    if p1 != 0 {
                        self.pc += p2;
                    } else {
                        self.pc += 1;
                    }
                }
                _ => panic!("unimplemented: {:?}", op),
            }
        }
        return mul_counter;
    }
}

#[derive(Debug, Clone, Copy)]
enum Param {
    R(char),
    I(i64),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(Param, Param),
    Set(Param, Param),
    Jnz(Param, Param),
    Sub(Param, Param),
}

fn get_val(reg: &mut HashMap<char, i64>, p: Param) -> i64 {
    match p {
        Param::R(p) => *reg.entry(p).or_insert(0),
        Param::I(i) => i,
    }
}

fn parse(input: Vec<String>) -> Vec<Op> {
    let re_binary = Regex::new(r"^(\S+?) (\S+?) (\S+?)$").unwrap();
    let mut r = Vec::new();

    for line in input.iter() {
        match re_binary.captures(line) {
            Some(cap) => {
                let param1 = parse_param(cap.get(2).unwrap().as_str());
                let param2 = parse_param(cap.get(3).unwrap().as_str());
                match cap.get(1).unwrap().as_str() {
                    "sub" => {
                        r.push(Op::Sub(param1, param2));
                        continue;
                    }
                    "jnz" => {
                        r.push(Op::Jnz(param1, param2));
                        continue;
                    }
                    "mul" => {
                        r.push(Op::Mul(param1, param2));
                        continue;
                    }
                    "set" => {
                        r.push(Op::Set(param1, param2));
                        continue;
                    }
                    _ => panic!("unknown binary op: {}", line),
                }
            }
            None => {}
        };
        panic!("parse error: {}", line);
    }

    return r;
}

fn parse_param(param: &str) -> Param {
    let re_immediate = Regex::new(r"^-?[0-9]+$").unwrap();
    if re_immediate.is_match(param) {
        return Param::I(param.parse().unwrap());
    }

    let re_register = Regex::new(r"^[a-z]$").unwrap();
    if re_register.is_match(param) {
        return Param::R(param.chars().nth(0).unwrap());
    }

    panic!("parse_param error: {}", param);
}
