pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1333);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 2046);
}

fn solve_part1(input: &Vec<String>) -> usize {
    let mut delta = 0;
    for s in input.iter() {
        // decode each line in the input and keep a running some of the difference.
        let literal_length = s.len();
        let decoded_length = decode(s).len();
        delta += literal_length - decoded_length;
    }
    return delta;
}

fn solve_part2(input: &Vec<String>) -> usize {
    let mut delta = 0;
    for s in input.iter() {
        // encode each line in the input and keep a running some of the difference.
        let literal_length = s.len();
        let encoded_length = encode(s).len();
        delta += encoded_length - literal_length;
    }
    return delta;
}

// Implement a decoder. It's not stricly necessary, but it's one way to solve things.
fn decode(input: &String) -> String {
    let input = input.as_bytes();
    let mut r = String::new();
    let mut i = 1;
    while i < input.len() - 1 {
        if input[i] == b'\\' {
            if input[i + 1] == b'\\' {
                r.push('\\');
                i += 2;
            } else if input[i + 1] == b'"' {
                r.push('"');
                i += 2;
            } else if input[i + 1] == b'x' {
                r.push('?'); // I'm being lazy!
                i += 4;
            }
        } else {
            r.push(input[i] as char);
            i += 1;
        }
    }
    return r;
}

// Implement an encoder. It's not stricly necessary, but it's one way to solve things.
fn encode(input: &String) -> String {
    let input = input.as_bytes();
    let mut r = String::new();
    r.push('"');
    for c in input.iter() {
        match *c {
            b'"' => r.push_str("\\\""),
            b'\\' => r.push_str("\\\\"),
            _ => r.push(*c as char),
        }
    }
    r.push('"');
    return r;
}
