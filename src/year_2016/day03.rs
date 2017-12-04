use regex::Regex;

pub fn run(input: &str) {
    assert_eq!(solve_part1("5 10 25\n10 5 25\n5 10 25\n5 10 25\n12 5 10"),
               1);
    let part1 = solve_part1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 982);

    assert_eq!(solve_part2("101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 \
                            602\n203 403 603"),
               6);
    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1826);
}

fn solve_part1(input: &str) -> usize {
    _solve(input, 1)
}

fn solve_part2(input: &str) -> usize {
    _solve(input, 3)
}

// The dir as a usize is a very unclean way to solve this puzzle.
fn _solve(input: &str, dir: usize) -> usize {
    let mut data: Vec<usize> = vec![];
    let re = Regex::new(r"\s*(\d+)").unwrap();
    for line in input.trim().split("\n") {
        // TODO: is there a simpler way to parse the input besides using a regexp? The regexp
        // implies some ugly unwrap() + parse() + expect().
        for cap in re.captures_iter(line) {
            data.push(cap.get(1).unwrap().as_str().parse().expect("expecting a number"));
        }
    }
    let mut i = 0;
    let mut total = 0;
    while i < data.len() {
        for j in 0..dir {
            let mut v = vec![data[i + j], data[i + j + dir], data[i + j + 2 * dir]];
            v.sort();
            if v[0] + v[1] > v[2] {
                total += 1;
            }
        }
        i += dir * 3;
    }
    total
}
