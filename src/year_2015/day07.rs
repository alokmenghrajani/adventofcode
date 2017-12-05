use regex::Captures;
use regex::Regex;
use std::collections::HashMap;

enum Signal {
    Other(String),
    And(String, String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
    Not(String),
}

pub fn run(input: Vec<String>) {
    let part1 = solve(&input, "a".to_string(), None);
    println!("part 1: {}", part1);
    assert_eq!(part1, 3176);

    let part2 = solve(&input, "a".to_string(), Some(3176));
    println!("part 2: {}", part2);
    assert_eq!(part2, 14710);
}

fn solve(input: &Vec<String>, signal: String, b: Option<u16>) -> u16 {
    let mut signals = HashMap::new();
    let re_other = Regex::new(r"^(\w+) -> (\D+)$").unwrap();
    let re_and = Regex::new(r"^(\w+) AND (\w+) -> (\D+)$").unwrap();
    let re_or = Regex::new(r"^(\w+) OR (\w+) -> (\D+)$").unwrap();
    let re_lshift = Regex::new(r"^(\w+) LSHIFT (\d+) -> (\D+)$").unwrap();
    let re_rshift = Regex::new(r"^(\w+) RSHIFT (\d+) -> (\D+)$").unwrap();
    let re_not = Regex::new(r"^NOT (\w+) -> (\D+)$").unwrap();

    // Process each line of input. It's a lot of code, but it's mostly stuff.
    for line in input.iter() {
        if try_parse(line,
                     &re_other,
                     &mut |cap| {
                         let name = cap.get(2).unwrap().as_str().to_string();
                         signals.insert(name,
                                        Signal::Other(cap.get(1).unwrap().as_str().to_string()));
                     }) {
            continue;
        }
        if try_parse(line,
                     &re_and,
                     &mut |cap| {
                         let name = cap.get(3).unwrap().as_str().to_string();
                         signals.insert(name,
                                        Signal::And(cap.get(1).unwrap().as_str().to_string(),
                                                    cap.get(2).unwrap().as_str().to_string()));
                     }) {
            continue;
        }
        if try_parse(line,
                     &re_or,
                     &mut |cap| {
                         let name = cap.get(3).unwrap().as_str().to_string();
                         signals.insert(name,
                                        Signal::Or(cap.get(1).unwrap().as_str().to_string(),
                                                   cap.get(2).unwrap().as_str().to_string()));
                     }) {
            continue;
        }
        if try_parse(line,
                     &re_lshift,
                     &mut |cap| {
            let name = cap.get(3).unwrap().as_str().to_string();
            signals.insert(name,
                           Signal::LShift(cap.get(1).unwrap().as_str().to_string(),
                                          cap.get(2).unwrap().as_str().parse().unwrap()));
        }) {
            continue;
        }
        if try_parse(line,
                     &re_rshift,
                     &mut |cap| {
            let name = cap.get(3).unwrap().as_str().to_string();
            signals.insert(name,
                           Signal::RShift(cap.get(1).unwrap().as_str().to_string(),
                                          cap.get(2).unwrap().as_str().parse().unwrap()));
        }) {
            continue;
        }
        if try_parse(line,
                     &re_not,
                     &mut |cap| {
                         let name = cap.get(2).unwrap().as_str().to_string();
                         signals.insert(name,
                           Signal::Not(cap.get(1).unwrap().as_str().to_string()));
                     }) {
            continue;
        }
        panic!("failed to parse: {}", line);
    }

    // recursively find the value of the desired signal
    let mut cache = HashMap::new();
    if b.is_some() {
        // override signal b by setting cache value
        cache.insert("b".to_string(), b.unwrap());
    }
    return get_val(&signals, &mut cache, &signal);
}

fn get_val(signals: &HashMap<String, Signal>, cache: &mut HashMap<String, u16>, s: &String) -> u16 {
    if cache.contains_key(s) {
        return *cache.get(s).unwrap();
    }
    let v: Result<u16, _> = s.parse();
    if v.is_ok() {
        return v.unwrap();
    }
    let r = match signals.get(s) {
        Some(&Signal::Other(ref s)) => get_val(signals, cache, s),
        Some(&Signal::And(ref s1, ref s2)) => {
            get_val(signals, cache, s1) & get_val(signals, cache, s2)
        }
        Some(&Signal::Or(ref s1, ref s2)) => {
            get_val(signals, cache, s1) | get_val(signals, cache, s2)
        }
        Some(&Signal::LShift(ref s, n)) => get_val(signals, cache, s) << n,
        Some(&Signal::RShift(ref s, n)) => get_val(signals, cache, s) >> n,
        Some(&Signal::Not(ref s)) => !get_val(signals, cache, s),
        None => panic!("unknown signal: {}", s),
    };
    cache.insert(s.clone(), r);
    return r;
}

fn try_parse<'t, F>(line: &'t String, re: &Regex, f: &mut F) -> bool
    where F: FnMut(Captures<'t>)
{
    match re.captures(line) {
        Some(cap) => {
            f(cap);
            return true;
        }
        None => return false,
    }
}
