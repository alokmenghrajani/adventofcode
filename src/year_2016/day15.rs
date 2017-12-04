// There are two ways to solve this problem: the mathematical and the algorithmic approach.
// From a math point of view, the Chinese Remainder Theorem exactly maps to this puzzle.
// From an algorithm point of view, we need to parse the input and look for the first integer i
// such that (i + disc_number + disc_starting_position) % disc_size == 0, for all discs.

extern crate regex;
use self::regex::Regex;

pub fn solve(input: &str) {
    let test_input = "Disc #1 has 5 positions; at time=0, it is at position 4.\nDisc #2 has 2 \
                      positions; at time=0, it is at position 1.";
    assert_eq!(_solve(test_input), 5);
    println!("part 1: {}", _solve(input));

    let part2 = format!("{}Disc #7 has 11 positions; at time=0, it is at position 0.",
                        input);
    println!("part 2: {}", _solve(&part2[..]));
}

struct Disc {
    index: usize,
    size: usize,
    start: usize,
}

fn _solve(input: &str) -> usize {
    // Parse the input and extract the disc index, size and position.
    let re = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)")
        .unwrap();
    let discs: Vec<Disc> = re.captures_iter(input)
        .map(|c| {
            Disc {
                index: c.at(1).unwrap().parse().unwrap(),
                size: c.at(2).unwrap().parse().unwrap(),
                start: c.at(3).unwrap().parse().unwrap(),
            }
        })
        .collect();

    // Find the first value which works for all discs.
    let mut i = 0;
    while discs.iter().any(|d| (i + d.index + d.start) % d.size != 0) {
        i += 1;
    }
    i
}
