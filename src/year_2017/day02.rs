use regex::Regex;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 47136);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 250);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    let re = Regex::new(r"\s+").unwrap();
    for line in input.iter() {
        // use a regular expression to split on whitespace
        let pieces: Vec<&str> = re.split(line).collect();
        // and convert each line into a vector of u64.
        let pieces: Vec<u64> = pieces.iter().map(|s| s.parse().unwrap()).collect();
        // the rest is easy, we have max() and min() functions.
        sum += pieces.iter().max().unwrap() - pieces.iter().min().unwrap();
    }
    return sum;
}

fn solve_part2(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    let re = Regex::new(r"\s+").unwrap();
    for line in input.iter() {
        // same as part1
        let pieces: Vec<&str> = re.split(line).collect();
        let pieces: Vec<u64> = pieces.iter().map(|s| s.parse().unwrap()).collect();

        // Look for numbers which divide each other. The only tricky part is to use
        // enumerate() so we can compare different entries.
        'outer: for (i, v1) in pieces.iter().enumerate() {
            for (j, v2) in pieces.iter().enumerate() {
                if (i != j) && (v1 % v2 == 0) {
                    sum += v1 / v2;
                    break 'outer;
                }
            }
        }
    }
    return sum;
}
