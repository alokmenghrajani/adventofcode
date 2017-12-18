use regex::Regex;
use std::collections::HashMap;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(input.clone());
    println!("part 1: {}", part1);
    assert_eq!(part1, 9423);

    let part2 = solve_part2(input.clone());
    println!("part 2: {}", part2);
    assert_eq!(part2, 7620);
}

fn solve_part1(input: Vec<String>) -> i64 {
    // Parse the input
    let prog = parse(input);

    // Run the code
    let p = Process::new(0, prog);
    return p.run_part1();
}

fn solve_part2(input: Vec<String>) -> u64 {
    // Parse the input
    let prog = parse(input);

    // Run the code. We let each process progress by one step until they deadlock
    let mut p0 = Process::new(0, prog.clone());
    let mut p1 = Process::new(1, prog.clone());

    loop {
        p0.step_part2(&mut p1);
        p1.step_part2(&mut p0);
        if p0.waiting && p1.waiting {
            break;
        }
    }

    return p1.num_sent;
}

struct Process {
    id: i64,
    pc: i64,
    waiting: bool,
    queue: Vec<i64>,
    reg: HashMap<char, i64>,
    num_sent: u64,
    prog: Vec<Op>,
}

impl Process {
    fn new(id: i64, prog: Vec<Op>) -> Process {
        let mut reg = HashMap::new();
        reg.insert('p', id);

        return Process {
            id: id,
            pc: 0,
            waiting: false,
            queue: Vec::new(),
            reg: reg,
            num_sent: 0,
            prog: prog,
        };
    }

    fn run_part1(mut self) -> i64 {
        let mut last_snd = None;
        loop {
            if (self.pc < 0) || (self.pc as usize >= self.prog.len()) {
                // we are out of range
                println!("  debug: pc is out of bounds");
                break;
            }
            let op = self.prog[self.pc as usize];
            match op {
                Op::Add(Param::R(p1), p2) => {
                    let t = get_val(&mut self.reg, Param::R(p1)) + get_val(&mut self.reg, p2);
                    self.reg.insert(p1, t);
                    self.pc += 1;
                }
                Op::Mod(Param::R(p1), p2) => {
                    let t = get_val(&mut self.reg, Param::R(p1)) % get_val(&mut self.reg, p2);
                    self.reg.insert(p1, t);
                    self.pc += 1;
                }
                Op::Mul(Param::R(p1), p2) => {
                    let t = get_val(&mut self.reg, Param::R(p1)) * get_val(&mut self.reg, p2);
                    self.reg.insert(p1, t);
                    self.pc += 1;
                }
                Op::Set(Param::R(p1), p2) => {
                    let t = get_val(&mut self.reg, p2);
                    self.reg.insert(p1, t);
                    self.pc += 1;
                }
                Op::Jgz(p1, p2) => {
                    let p1 = get_val(&mut self.reg, p1);
                    let p2 = get_val(&mut self.reg, p2);
                    if p1 > 0 {
                        self.pc += p2;
                    } else {
                        self.pc += 1;
                    }
                }
                Op::Snd(p) => {
                    let p = get_val(&mut self.reg, p);
                    last_snd = Some(p);
                    self.pc += 1;
                }
                Op::Rcv(p) => {
                    let p = get_val(&mut self.reg, p);
                    if p != 0 {
                        // we are done
                        break;
                    }
                    self.pc += 1;
                }
                _ => panic!("unimplemented: {:?}", op),
            }
        }

        return last_snd.unwrap();
    }

    fn step_part2(&mut self, other: &mut Process) {
        self.waiting = false;

        if (self.pc < 0) || (self.pc as usize >= self.prog.len()) {
            // we are out of range
            println!("  debug: {} is out of bounds", self.id);
            self.waiting = true;
            return;
        }
        let op = self.prog[self.pc as usize];
        match op {
            Op::Add(Param::R(p1), p2) => {
                let t = get_val(&mut self.reg, Param::R(p1)) + get_val(&mut self.reg, p2);
                self.reg.insert(p1, t);
                self.pc += 1;
            }
            Op::Mod(Param::R(p1), p2) => {
                let t = get_val(&mut self.reg, Param::R(p1)) % get_val(&mut self.reg, p2);
                self.reg.insert(p1, t);
                self.pc += 1;
            }
            Op::Mul(Param::R(p1), p2) => {
                let t = get_val(&mut self.reg, Param::R(p1)) * get_val(&mut self.reg, p2);
                self.reg.insert(p1, t);
                self.pc += 1;
            }
            Op::Set(Param::R(p1), p2) => {
                let t = get_val(&mut self.reg, p2);
                self.reg.insert(p1, t);
                self.pc += 1;
            }
            Op::Jgz(p1, p2) => {
                let p1 = get_val(&mut self.reg, p1);
                let p2 = get_val(&mut self.reg, p2);
                if p1 > 0 {
                    self.pc += p2;
                } else {
                    self.pc += 1;
                }
            }
            Op::Snd(p) => {
                let p = get_val(&mut self.reg, p);
                other.queue.push(p);
                self.num_sent += 1;
                self.pc += 1;
            }
            Op::Rcv(Param::R(p)) => {
                if self.queue.len() == 0 {
                    self.waiting = true;
                    return;
                }
                let t = self.queue.remove(0);
                self.reg.insert(p, t);
                self.pc += 1;
            }
            _ => panic!("unimplemented: {:?}", op),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Param {
    R(char),
    I(i64),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(Param, Param),
    Mod(Param, Param),
    Mul(Param, Param),
    Set(Param, Param),
    Jgz(Param, Param),
    Rcv(Param),
    Snd(Param),
}

fn get_val(reg: &mut HashMap<char, i64>, p: Param) -> i64 {
    match p {
        Param::R(p) => *reg.entry(p).or_insert(0),
        Param::I(i) => i,
    }
}

fn parse(input: Vec<String>) -> Vec<Op> {
    let re_unary = Regex::new(r"^(\S+?) (\S+?)$").unwrap();
    let re_binary = Regex::new(r"^(\S+?) (\S+?) (\S+?)$").unwrap();
    let mut r = Vec::new();

    for line in input.iter() {
        match re_binary.captures(line) {
            Some(cap) => {
                let param1 = parse_param(cap.get(2).unwrap().as_str());
                let param2 = parse_param(cap.get(3).unwrap().as_str());
                match cap.get(1).unwrap().as_str() {
                    "add" => {
                        r.push(Op::Add(param1, param2));
                        continue;
                    }
                    "jgz" => {
                        r.push(Op::Jgz(param1, param2));
                        continue;
                    }
                    "mod" => {
                        r.push(Op::Mod(param1, param2));
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
        match re_unary.captures(line) {
            Some(cap) => {
                let param = parse_param(cap.get(2).unwrap().as_str());
                match cap.get(1).unwrap().as_str() {
                    "rcv" => {
                        r.push(Op::Rcv(param));
                        continue;
                    }
                    "snd" => {
                        r.push(Op::Snd(param));
                        continue;
                    }
                    _ => panic!("unknown unary op: {}", line),
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
