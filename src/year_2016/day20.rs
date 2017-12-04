extern crate regex;

use self::regex::Regex;

// This puzzle can be solved by using the right data structure and merging overlapping ranges of
// numbers. I however picked a less efficient but much simpler approach:
// Starting from the min value, keep incrementing a counter i. If there's a [lower-upper] range,
// such that lower <= i <= upper, then update the counter's value to be upper+1. Otherwise, i is
// one of the whitelisted values we are interested in.
//
// This implementation uses an iterator, which allows nice and efficient code reuse between the
// two parts.
pub fn solve(input: &str) {
    let test_input = "5-8\n0-2\n4-7";
    assert_eq!(part1(test_input, 0, 9), 3);
    println!("part 1: {}", part1(input, 0, 4294967295));

    assert_eq!(part2(test_input, 0, 9), 2);
    println!("part 2: {}", part2(input, 0, 4294967295));
}

struct Day20Iterator {
    ranges: Vec<(isize, isize)>,
    i: isize,
    max: isize,
}

impl Iterator for Day20Iterator {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        while self.i <= self.max {
            if let Some(&(_, upper)) =
                self.ranges
                    .iter()
                    .find(|bounds| bounds.0 <= self.i && self.i <= bounds.1) {
                self.i = upper + 1;
            } else {
                let r = Some(self.i);
                self.i += 1;
                return r;
            }
        }
        None
    }
}

fn day20iterator(input: &str, min: isize, max: isize) -> Day20Iterator {
    // convert input to a tuple of lower-upper bounds.
    let mut ranges: Vec<(isize, isize)> = vec![];
    lazy_static! {
        static ref P: Regex = Regex::new(r"^(\d+)-(\d+)").unwrap();
    }
    for line in input.trim().split('\n') {
        let cap = P.captures(line).unwrap();
        ranges.push((cap.get(1).unwrap().as_str().parse().unwrap(),
                     cap.get(2).unwrap().as_str().parse().unwrap()));
    }

    Day20Iterator {
        ranges: ranges,
        i: min,
        max: max,
    }
}

fn part1(input: &str, min: isize, max: isize) -> isize {
    day20iterator(input, min, max).next().unwrap()
}

fn part2(input: &str, min: isize, max: isize) -> usize {
    day20iterator(input, min, max).count()
}
