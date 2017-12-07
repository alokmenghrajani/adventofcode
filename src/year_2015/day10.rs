pub fn run(input: &str) {
    println!("pro-tip: compile with --release!");

    let part1 = solve(input, 40);
    println!("part 1: {}", part1);
    assert_eq!(part1, 329356);

    let part2 = solve(input, 50);
    println!("part 2: {}", part2);
    assert_eq!(part2, 4666278);
}

fn solve(input: &str, iterations: usize) -> usize {
    let mut i: Vec<u8> = input.as_bytes().iter().map(|c| *c).collect();
    for _ in 0..iterations {
        let mut t = Vec::new();
        let mut c = (i[0], 0);
        for j in 0..i.len() {
            if i[j] == c.0 {
                c = (c.0, c.1 + 1)
            } else {
                t.extend_from_slice(c.1.to_string().as_bytes());
                t.push(c.0);
                c = (i[j], 1);
            }
        }
        t.extend_from_slice(c.1.to_string().as_bytes());
        t.push(c.0);

        i.truncate(0);
        i.extend_from_slice(&t);
    }
    return i.len();
}
