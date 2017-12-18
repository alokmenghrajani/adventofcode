use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    let part1 = solve(&input, false);
    println!("part 1: {}", part1);
    assert_eq!(part1, 664);

    let part2 = solve(&input, true);
    println!("part 2: {}", part2);
    assert_eq!(part2, 640);
}

fn solve(input: &Vec<String>, extra: bool) -> i64 {
    // parse the input
    let mut happiness: HashMap<(&str, &str), i64> = HashMap::new();
    let mut people: HashSet<&str> = HashSet::new();

    let re =
        Regex::new(r"^(.+?) would (gain|lose) (\d+?) happiness units by sitting next to (.+?)\.$")
            .unwrap();
    for line in input.iter() {
        match re.captures(line) {
            Some(cap) => {
                let p1 = cap.get(1).unwrap().as_str();
                let p2 = cap.get(4).unwrap().as_str();
                people.insert(p1);
                people.insert(p2);
                let sign = if cap.get(2).unwrap().as_str() == "gain" {
                    1
                } else {
                    -1
                };
                let units: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
                happiness.insert((p1, p2), units * sign);
            }
            None => {
                panic!("invalid input: {}", line);
            }
        }
    }

    // handle part2
    if extra {
        let you = "you";
        for p in people.iter() {
            happiness.insert((p, you), 0);
            happiness.insert((you, p), 0);
        }
        people.insert(you);
    }

    // we "anchor" one person, because the table is round.
    let p = people.iter().next().unwrap().clone();
    let mut seen = HashSet::new();
    seen.insert(p);

    // recursively explore the search space.
    let max = find_max(&happiness, p, p, &people, &mut seen);
    return max;
}

fn find_max<'a>(happiness: &HashMap<(&'a str, &'a str), i64>,
                first: &'a str,
                prev: &'a str,
                people: &HashSet<&'a str>,
                seen: &mut HashSet<&'a str>)
                -> i64 {
    if seen.len() == people.len() {
        // we are done. We have to adjust for first and last people sitting next to each other
        return happiness.get(&(first, prev)).unwrap() + happiness.get(&(prev, first)).unwrap();
    }
    let mut max = None;
    for p in people.iter() {
        if seen.contains(p) {
            continue;
        }
        let p = p.clone();
        seen.insert(p);
        let mut v = find_max(happiness, first, p, people, seen);
        seen.remove(p);
        v += *happiness.get(&(prev, p)).unwrap();
        v += *happiness.get(&(p, prev)).unwrap();
        max = match max {
            None => Some(v),
            Some(max) => if v > max { Some(v) } else { Some(max) },
        }
    }
    return max.unwrap();
}
