pub fn run(input: &[u8]) {
    let part1 = part1(input);
    println!("part1: {}", part1);

    let part2 = part2(input);
    println!("part2: {}", part2);
}

fn part1(buf: &[u8]) -> i64 {
    let mut floor: i64 = 0;
    // loop over every character
    for i in 0..buf.len() {
        if buf[i] == b'(' {
            // increment floor when we see a '('
            floor += 1;
        } else if buf[i] == b')' {
            // decrement floor when we see a ')'
            floor -= 1;
        } else {
            panic!("invalid input: {}", buf[i]);
        }
    }
    return floor;
}

fn part2(buf: &[u8]) -> usize {
    let mut floor: i64 = 0;
    // loop over every character
    for i in 0..buf.len() {
        if buf[i] == b'(' {
            // increment floor when we see a '('
            floor += 1;
        } else if buf[i] == b')' {
            // decrement floor when we see a ')'
            floor -= 1;
        } else {
            panic!("invalid input: {}", buf[i]);
        }
        if floor == -1 {
            // return the current position when we hit the basement
            return i;
        }
    }
    panic!("Santa never enters basement.");
}
