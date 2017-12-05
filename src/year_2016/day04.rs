use regex::Regex;
use std::collections::HashMap;

pub fn run(input: &str) {
    assert_eq!(checksum("aaaaa-bbb-z-y-x"), "abxyz");
    assert_eq!(checksum("a-b-c-d-e-f-g-h"), "abcde");
    assert_eq!(checksum("not-a-real-room"), "oarel");
    assert_ne!(checksum("totally-real-room"), "decoy");

    let part1 = solve_part1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 137896);

    assert_eq!(rotate("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");

    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 501);
}

fn solve_part1(input: &str) -> usize {
    _solve(input).0
}

fn solve_part2(input: &str) -> usize {
    _solve(input).1.expect("failed to find North Pole object")
}

fn _solve(input: &str) -> (usize, Option<usize>) {
    let mut total = 0;
    let mut north_pole = None;
    for line in input.trim().split("\n") {
        let re = Regex::new(r"([a-z-]*)-(\d+)\[([a-z]{5})\]").unwrap();
        let cap = re.captures(line).unwrap();
        let room = cap.get(1).unwrap().as_str();
        let id = cap.get(2).unwrap().as_str().parse().unwrap();
        let chksum = cap.get(3).unwrap().as_str();
        if checksum(room) == chksum {
            total += id;
        }
        if rotate(room, id) == "northpole object storage" {
            north_pole = Some(id);
        }
    }
    (total, north_pole)
}

fn from_char(c: char) -> usize {
    return ((c as u8) - b'a') as usize;
}

fn to_char(i: usize) -> char {
    return (((i % 26) as u8) + b'a') as char;
}

fn rotate(s: &str, offset: usize) -> String {
    return s.chars()
        .map(|c| if c == '-' {
            ' '
        } else {
            to_char(from_char(c) + offset)
        })
        .collect();
}

fn checksum(s: &str) -> String {
    let mut sums = HashMap::new();
    for c in s.chars() {
        if c == '-' {
            continue;
        }
        *sums.entry(from_char(c)).or_insert(0) += 1;
    }
    // Convert the map<K, V> into a vector of (K, V). We can then sort the vector and return the
    // first 5 elements.
    let mut t: Vec<(&usize, &i32)> = sums.iter().collect();
    t.sort_by(|v1, v2| if v1.1 == v2.1 {
        v1.0.cmp(v2.0)
    } else {
        v2.1.cmp(v1.1)
    });
    // If we try to use .map() and push characters in a string, we have to consume the iterator.
    t.iter().take(5).fold(String::new(), |acc, v| format!("{}{}", acc, to_char(*v.0)))
}
