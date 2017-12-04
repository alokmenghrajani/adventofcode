extern crate crypto;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn solve(input: &str) {
    assert_eq!(part1("abc"), "18f47a30");
    println!("part 1: {}", part1(input));

    assert_eq!(part2("abc"), "05ace8e3");
    println!("part 2: {}", part2(input));
}

fn part1(door: &str) -> String {
    // compute MD5s until we get 8 hashes with 5 leading zeros.
    // we store each character we find in a String.
    let mut res = String::new();
    let mut i = 0;
    while res.len() < 8 {
        match compute_md5(door, i) {
            Some(s) => {
                res.push(s.chars().nth(5).unwrap());
            }
            None => (),
        }
        i += 1;
    }
    res
}

fn part2(door: &str) -> String {
    // compute MD5s until we fill our array.
    let mut res = [b'-'; 8].to_vec();
    let mut found = 0;
    let mut i = 0;
    while found < 8 {
        match compute_md5(door, i) {
            Some(s) => {
                let x = unhex(s.bytes().nth(5).unwrap());
                if x < 8 && res[x as usize] == b'-' {
                    res[x as usize] = s.bytes().nth(6).unwrap();
                    found += 1;
                }
            }
            None => (),
        }
        i += 1;
    }
    String::from_utf8(res).unwrap()
}

// I shouldn't need this...
fn unhex(c: u8) -> u8 {
    if (c >= b'0') && (c <= b'9') {
        return c - b'0';
    }
    return c - b'a' + 10;
}

// md5 computation is very slow if you don't compile the code with --release so use
// `cargo run 5 --release` for this level.
// It would be interesting to wrap the md5 computation in a future and see how much parallelism
// we get out of it.
fn compute_md5(door: &str, i: i32) -> Option<String> {
    let mut md5 = Md5::new();
    md5.reset();
    md5.input_str(door);
    md5.input_str(&i.to_string());
    let s = md5.result_str();
    if s.starts_with("00000") {
        return Some(s);
    }
    None
}
