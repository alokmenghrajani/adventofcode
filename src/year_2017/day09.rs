pub fn run(input: &[u8]) {
    let part1 = solve(input).0;
    println!("part 1: {}", part1);
    assert_eq!(part1, 16689);

    let part2 = solve(input).1;
    println!("part 2: {}", part2);
    assert_eq!(part2, 7982);
}

fn solve(input: &[u8]) -> (u64, u64) {
    // Iterate thru the input and keep track of:
    // - what is the current depth of '{' and '}'. Increment a sum each time the depth increases.
    // - whether we are processing garbage and how much of it.

    let mut score = 0;
    let mut depth = 0;
    let mut garbage = false;
    let mut total_garbage = 0;

    let mut i = 0;
    while i < input.len() {
        let c = input[i] as char;
        if garbage {
            match c {
                '>' => garbage = false,
                '!' => i += 1,
                _ => total_garbage += 1,
            };
        } else {
            match c {
                '{' => {
                    depth += 1;
                    score += depth;
                }
                '}' => depth -= 1,
                '<' => garbage = true,
                _ => {}
            };
        }
        i += 1;
    }

    return (score, total_garbage);
}
