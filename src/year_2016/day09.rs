extern crate regex;

use self::regex::Regex;

pub fn solve(input: &str) {
    assert_eq!(part1("ADVENT"), 6);
    assert_eq!(part1("A(1x5)BC"), 7);
    assert_eq!(part1("A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(part1("(6x1)(1x3)A"), 6);
    assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
    println!("part 1: {}", part1(input));

    assert_eq!(part2("(3x3)XYZ"), 9);
    assert_eq!(part2("X(8x2)(3x3)ABCY"), 20);
    assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
               445);
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    _solve(input, false)
}

fn part2(input: &str) -> usize {
    _solve(input, true)
}

// Recursively compute the length of our input.
fn _solve(input: &str, recursive: bool) -> usize {
    lazy_static! {
            static ref NORMAL: Regex = Regex::new("^[A-Z]+").unwrap();
            static ref REPEAT: Regex = Regex::new(r"^\((\d+)x(\d+)\)").unwrap();
            static ref WHITESPACE: Regex = Regex::new(r"^\s+").unwrap();
    }
    // Recursion ends when we have consumed the entire input.
    if input.len() == 0 {
        return 0;
    }
    // Check if we have a character in the A-Z range.
    if let Some(n) = NORMAL.captures(input) {
        let l = n.get(0).unwrap().as_str().len();
        return l + _solve(&input[l..], recursive);
    }
    // Check if we have a repetition pattern.
    if let Some(r) = REPEAT.captures(input) {
        let skip = r.get(0).unwrap().as_str().len();
        let t: usize = r.get(1).unwrap().as_str().parse().unwrap();
        let c: usize = r.get(2).unwrap().as_str().parse().unwrap();

        if recursive {
            // for part2, we call the recursion on both pieces
            return c * _solve(&input[skip..skip + t], recursive) +
                   _solve(&input[skip + t..], recursive);
        } else {
            // for part1, we only call the recursion on the second piece
            return c * t + _solve(&input[skip + t..], recursive);
        }
    }
    // Ignore whitespaces
    if WHITESPACE.is_match(input) {
        return 0;
    }
    panic!("unexpected input");
}
