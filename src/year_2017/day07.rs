use regex::Regex;
use std::collections::HashMap;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, "vtzay");

    let part2 = solve_part2(&input, "vtzay");
    println!("part 2: {}", part2);
    assert_eq!(part2, 910);
}

fn solve_part1(input: &Vec<String>) -> &str {
    // parse the input into a datastructure
    // we'll store the weights in one map and the tree structure in another.
    let mut weights: HashMap<&str, u64> = HashMap::new();
    let mut tree: HashMap<&str, &str> = HashMap::new();

    let re1 = Regex::new(r"^(.+?) \((\d+)\) -> (.+)$").unwrap();
    let re2 = Regex::new(r"^(.+?) \((\d+)\)$").unwrap();
    for line in input.iter() {
        let cap = re1.captures(line);
        if cap.is_some() {
            let cap = cap.unwrap();
            let parent = cap.get(1).unwrap().as_str();
            let weight: u64 = cap.get(2).unwrap().as_str().parse().unwrap();
            let children = cap.get(3).unwrap().as_str();
            weights.insert(parent, weight);
            for child in children.split(", ") {
                tree.insert(child, parent);
            }
            continue;
        }
        let cap = re2.captures(line);
        if cap.is_some() {
            let cap = cap.unwrap();
            let parent = cap.get(1).unwrap().as_str();
            let weight = cap.get(2).unwrap().as_str().parse().unwrap();
            weights.insert(parent, weight);
            continue;
        }
        panic!("invalid input: {}", line);
    }

    // starting from a random node, walk up the tree
    let mut s: &str = weights.keys().next().unwrap();
    while tree.contains_key(s) {
        s = tree.get(s).unwrap();
    }
    return s;
}

fn solve_part2(input: &Vec<String>, root: &str) -> i64 {
    // parse the input into a datastructure
    // we'll store the weights in one map and the tree structure in another. The tree is reversed
    // from part1.
    let mut weights: HashMap<&str, i64> = HashMap::new();
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();

    let re1 = Regex::new(r"^(.+?) \((\d+)\) -> (.+)$").unwrap();
    let re2 = Regex::new(r"^(.+?) \((\d+)\)$").unwrap();
    for line in input.iter() {
        let cap = re1.captures(line);
        if cap.is_some() {
            let cap = cap.unwrap();
            let parent = cap.get(1).unwrap().as_str();
            let weight: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
            let children = cap.get(3).unwrap().as_str();
            weights.insert(parent, weight);
            tree.insert(parent, children.split(", ").collect());
            continue;
        }
        let cap = re2.captures(line);
        if cap.is_some() {
            let cap = cap.unwrap();
            let parent = cap.get(1).unwrap().as_str();
            let weight = cap.get(2).unwrap().as_str().parse().unwrap();
            weights.insert(parent, weight);
            continue;
        }
        panic!("invalid input: {}", line);
    }

    // starting from the root node, recursively check each child.
    match check(&tree, &weights, root) {
        Either::Solution(s) => return s,
        _ => panic!("failed to find a solution."),
    }
}

enum Either {
    Weight(i64),
    Solution(i64),
}

// The check function calculates a node's weight + all its children.
fn check(tree: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, i64>, node: &str) -> Either {
    match tree.get(node) {
        Some(children) => {
            let mut r = vec![];
            // build an array of weights, by recursively calling check.
            for child in children.iter() {
                match check(&tree, &weights, child) {
                    s @ Either::Solution(_) => return s,
                    Either::Weight(child_weight) => r.push(child_weight),
                }
            }
            // see if any node in the array is mis-balanced
            for i in 0..children.len() {
                let other1 = (i + 1) % children.len();
                let other2 = (i + 2) % children.len();
                if r[i] != r[other1] && r[i] != r[other2] {
                    // we found a discrepency!
                    return Either::Solution(weights.get(children[i]).unwrap() + r[other1] - r[i]);
                }
            }
            // everything is good, return the sum of this child weights + this node's
            return Either::Weight(r[0] * children.len() as i64 + weights.get(node).unwrap());
        }
        None => {
            // leaf node. Simply return its weight.
            return Either::Weight(*weights.get(node).unwrap());
        }
    }
}
