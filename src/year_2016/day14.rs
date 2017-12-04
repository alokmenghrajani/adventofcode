extern crate crypto;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;
use std::collections::HashMap;

pub fn solve(input: &str) {
    assert_eq!(part1("abc", 1), (b'e', 39, 816));
    println!("part 1: {}", part1(input, 64).1);

    assert_eq!(part2("abc", 1), (b'e', 10, 89));
    println!("part 2: {}", part2(input, 64).1);
}

fn part1(input: &str, n: usize) -> (u8, usize, usize) {
    _solve(input, n, 1)
}

fn part2(input: &str, n: usize) -> (u8, usize, usize) {
    _solve(input, n, 2017)
}

fn _solve(input: &str, n: usize, rounds: usize) -> (u8, usize, usize) {
    let mut i = 0;
    let mut found = 0;
    let mut cache = HashMap::new();
    loop {
        let hash = compute_md5(input, i, rounds, &mut cache).bytes().collect();
        if let Some(c) = find_three_consecutive(hash) {
            if let Some(k) = find_five_consecutive(input, i, c, rounds, &mut cache) {
                found += 1;
                if found == n {
                    return (c, i, k);
                }
            }
        }
        i += 1;
    }
}

fn find_three_consecutive(hash: Vec<u8>) -> Option<u8> {
    for i in 0..(hash.len() - 2) {
        if hash[i] == hash[i + 1] && hash[i] == hash[i + 2] {
            return Some(hash[i]);
        }
    }
    None
}

fn find_five_consecutive(salt: &str,
                         offset: usize,
                         c: u8,
                         rounds: usize,
                         cache: &mut HashMap<usize, String>)
                         -> Option<usize> {
    for j in 1..1001 {
        let hash: Vec<u8> = compute_md5(salt, offset + j, rounds, cache).bytes().collect();
        for i in 0..(hash.len() - 4) {
            if hash[i] == c && hash[i] == hash[i + 1] && hash[i] == hash[i + 2] &&
               hash[i] == hash[i + 3] && hash[i] == hash[i + 4] {
                return Some(offset + j);
            }
        }
    }
    None
}

fn compute_md5(salt: &str, i: usize, rounds: usize, cache: &mut HashMap<usize, String>) -> String {
    if let Some(r) = cache.get(&i) {
        return r.clone();
    }
    let mut md5 = Md5::new();
    let mut r = format!("{}{}", salt, i);
    for _ in 0..rounds {
        md5.reset();
        md5.input_str(&r[..]);
        r = md5.result_str();
    }
    cache.insert(i, r.clone());
    r
}
