use common::sfy::Sfy;
use fancy_regex::*;

pub fn run(input: &str) {
    println!("pro-tip: compile with --release!");

    let part1 = solve(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, "vzbxxyzz");

    let part2 = solve("vzbxxyzz");
    println!("part 2: {}", part2);
    assert_eq!(part2, "vzcaabcc");
}

fn solve(input: &str) -> String {
    // loop until we find a string which is valid
    let len = input.len();
    let mut s = Vec::from(input.as_bytes());
    next(&mut s, len - 1);

    let re = Regex::new(r"(.)\1.*(.)\2").unwrap();

    while !valid(&s, &re) {
        next(&mut s, len - 1);
    }
    return s.sfy();
}

// recursively increment the input
fn next(input: &mut [u8], offset: usize) {
    if input[offset] == b'z' {
        input[offset] = b'a';
        next(input, offset - 1);
    } else {
        input[offset] = input[offset] + 1;
        if (input[offset] == b'i') || (input[offset] == b'o') || (input[offset] == b'l') {
            // mini-optimization
            input[offset] = input[offset] + 1;
            for i in (offset + 1)..input.len() {
                input[i] = b'a';
            }
        }
    }
}

fn valid(input: &[u8], re: &Regex) -> bool {
    // use a helper function to find a straight of 3 letters
    if !has_straight(&input) {
        return false;
    }

    // use a a loop to check if input contains i, o or l.
    for i in input.iter() {
        if (*i == b'i') || (*i == b'o') || (*i == b'l') {
            return false;
        }
    }

    // use regexp to check for non-overlapping pairs
    if !re.is_match(&input.sfy()).unwrap() {
        return false;
    }

    return true;
}

fn has_straight(input: &[u8]) -> bool {
    for i in 0..input.len() - 2 {
        if (input[i] == input[i + 1] - 1) && (input[i] == input[i + 2] - 2) {
            return true;
        }
    }
    return false;
}
