extern crate regex;

use self::regex::Regex;

// Part 1 is pretty straighforward to implement. I used iterators as much as possible but it might
// not be the ideal apporach to manipulate a string. It would be interesting to compare the Rust
// code with a solution in Ruby, PHP or JS.
//
// For part 2, we can test all possible inputs. Reversing part1 is doable, but RotLetter can
// potentially have multiple or 0 reverse answers so we might have to explore a search space. It
// turned out that the inputs were carefully crafted for not needing that.

pub fn solve(input: &str, s1: &str, s2: &str) {
    assert_eq!(part1("swap position 4 with position 0", "abcde"), "ebcda");
    assert_eq!(part1("swap letter d with letter b", "ebcda"), "edcba");
    assert_eq!(part1("reverse positions 0 through 4", "edcba"), "abcde");
    assert_eq!(part1("rotate left 1 step", "abcde"), "bcdea");
    assert_eq!(part1("move position 1 to position 4", "bcdea"), "bdeac");
    assert_eq!(part1("move position 3 to position 0", "bdeac"), "abdec");
    assert_eq!(part1("rotate based on position of letter b", "abdec"),
               "ecabd");
    assert_eq!(part1("rotate based on position of letter d", "ecabd"),
               "decab");
    assert_eq!(part1("rotate right 2 steps", "decab"), "abdec");
    println!("part 1: {}", part1(input, s1));

    println!("part 2: {}", part2(input, s2));
}

enum Op {
    SwapPos(usize, usize),
    SwapLetter(u8, u8),
    RotLeft(usize),
    RotRight(usize),
    RotLetter(u8),
    Rev(usize, usize),
    Move(usize, usize),
}

fn part1(input: &str, s: &str) -> String {
    // parse the input
    let mut ops = vec![];
    for line in input.trim().split('\n') {
        lazy_static! {
            static ref SWAP_POS: Regex = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
            static ref SWAP_LETTER: Regex = Regex::new(r"swap letter (\w) with letter (\w)").unwrap();
            static ref ROT_LEFT: Regex = Regex::new(r"rotate left (\d+) step").unwrap();
            static ref ROT_RIGHT: Regex = Regex::new(r"rotate right (\d+) step").unwrap();
            static ref ROT_LETTER: Regex = Regex::new(r"rotate based on position of letter (\w)").unwrap();
            static ref REV: Regex = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
            static ref MOVE: Regex = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
        }
        if let Some(cap) = SWAP_POS.captures(line) {
            ops.push(Op::SwapPos(cap.at(1).unwrap().parse().unwrap(),
                                 cap.at(2).unwrap().parse().unwrap()));
        } else if let Some(cap) = SWAP_LETTER.captures(line) {
            ops.push(Op::SwapLetter(cap.at(1).unwrap().as_bytes()[0],
                                    cap.at(2).unwrap().as_bytes()[0]));
        } else if let Some(cap) = ROT_LEFT.captures(line) {
            ops.push(Op::RotLeft(cap.at(1).unwrap().parse().unwrap()));
        } else if let Some(cap) = ROT_RIGHT.captures(line) {
            ops.push(Op::RotRight(cap.at(1).unwrap().parse().unwrap()));
        } else if let Some(cap) = ROT_LETTER.captures(line) {
            ops.push(Op::RotLetter(cap.at(1).unwrap().as_bytes()[0]));
        } else if let Some(cap) = REV.captures(line) {
            ops.push(Op::Rev(cap.at(1).unwrap().parse().unwrap(),
                             cap.at(2).unwrap().parse().unwrap()));
        } else if let Some(cap) = MOVE.captures(line) {
            ops.push(Op::Move(cap.at(1).unwrap().parse().unwrap(),
                              cap.at(2).unwrap().parse().unwrap()));
        } else {
            panic!();
        }
    }

    let mut r: Vec<u8> = s.bytes().collect();

    // apply each Op
    for op in ops.iter() {
        match op {
            &Op::SwapPos(x, y) => r.swap(x, y),
            &Op::SwapLetter(a, b) => {
                r = r.iter()
                    .map(|x| if *x == a {
                        b
                    } else if *x == b {
                        a
                    } else {
                        *x
                    })
                    .collect();
            }
            &Op::RotLeft(x) => {
                let other = r.split_off(x);
                r = other.iter().chain(r.iter()).map(|x| *x).collect();
            }
            &Op::RotRight(x) => {
                let l = r.len();
                let other = r.split_off(l - x);
                r = other.iter().chain(r.iter()).map(|x| *x).collect();
            }
            &Op::RotLetter(a) => {
                let mut t = r.iter().position(|x| *x == a).unwrap();
                if t >= 4 {
                    t += 1;
                }
                t += 1;
                let l = r.len();
                let other = r.split_off((2 * l - t) % l);
                r = other.iter().chain(r.iter()).map(|x| *x).collect();
            }
            &Op::Rev(x, y) => {
                // I tried to only use iterators, but the result is unreadable. I'm keeping it this
                // way to showcase the crazyness.
                r = r.iter()
                    .take(x)
                    .chain(r.iter()
                        .skip(x)
                        .take(y - x + 1)
                        .collect::<Vec<&u8>>()
                        .iter()
                        .map(|x| *x)
                        .rev())
                    .chain(r.iter().skip(y + 1))
                    .map(|x| *x)
                    .collect()
            }
            &Op::Move(x, y) => {
                let t = r[x];
                r = r.iter().take(x).chain(r.iter().skip(x + 1)).map(|x| *x).collect();
                r.insert(y, t);
            }
        }
    }

    String::from_utf8(r).unwrap()
}

fn part2(input: &str, s: &str) -> String {
    // create a vector of all permutations
    let s = s.to_string();
    let v = s.bytes().collect();
    let mut p = vec![];
    permutations(&v, 0, vec![], &mut p);

    // look for the first permutation which gives the desired result.
    let solution = p.iter()
        .map(|x| (x.clone(), part1(input, &x.clone())))
        .find(|x| *x.1 == s);
    solution.unwrap().0
}

// Recursively generates all permutations for a given string s.
// note: it would be way more efficient to convert this function into an iterator.
fn permutations(s: &Vec<u8>, offset: usize, prefix: Vec<u8>, r: &mut Vec<String>) {
    //    println!("in permutations: {:?}, {:?}, {:?}", prefix, s, r);
    if offset == s.len() {
        r.push(String::from_utf8(prefix).unwrap());
    } else {
        let c = s[offset];
        for i in 0..(prefix.len() + 1) {
            let mut t = prefix.clone();
            t.insert(i, c);
            permutations(&s, offset + 1, t, r);
        }
    }
}
