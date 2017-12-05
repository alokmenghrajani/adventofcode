use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn run(input: &str) {
    println!("pro-tip: compile with --release");

    assert_eq!(solve_part1("abc"), "18f47a30");
    let part1 = solve_part1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, "4543c154");

    assert_eq!(solve_part2("abc"), "05ace8e3");
    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, "1050cbbd");
}

fn solve_part1(door: &str) -> String {
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

fn solve_part2(door: &str) -> String {
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
