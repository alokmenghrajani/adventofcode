use regex::Regex;

pub fn run(input: Vec<String>) {
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
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
    println!("part1: {}", sum);
}

fn part2(input: &Vec<String>) {
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
    println!("part2: {}", sum);
}
