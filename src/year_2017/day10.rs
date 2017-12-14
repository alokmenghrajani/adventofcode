use common::sfy::Sfy;

pub fn run(input: &[u8]) {
    let part1 = solve_part1(256, input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 212);

    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, "96de9657665675b51cd03f0b3528ba26");
}

fn solve_part1(len: u64, input: &[u8]) -> u64 {
    // Intialize the circle.
    let mut hash = Vec::with_capacity(len as usize);
    for i in 0..len {
        hash.push(i);
    }

    let mut pos = 0;
    let mut skip = 0;

    // convert input into a list of numbers
    let s = input.sfy();
    let inputs: Vec<u64> = s.split(",").map(|e| e.parse().unwrap()).collect();

    for input in inputs.iter() {
        // reverse [pos, pos + input]
        let t = hash.clone();
        for i in pos..(pos + input) {
            hash[(i % len) as usize] = t[((2 * pos + input - 1 - i) % len) as usize];
        }
        // increase pos
        pos += input + skip;
        skip += 1;
    }

    return hash[0] * hash[1];
}

fn solve_part2(input: &[u8]) -> String {
    return knot_hash(input);
}

pub fn knot_hash(input: &[u8]) -> String {
    // append 17, 31, 73, 47, 23 to the input
    let mut inputs = Vec::new();
    inputs.extend_from_slice(input);
    inputs.extend_from_slice(&[17, 31, 73, 47, 23]);

    // Intialize the circle.
    let mut hash = Vec::with_capacity(256);
    for i in 0..256 {
        hash.push(i);
    }

    let mut pos = 0;
    let mut skip = 0;

    // Compute sparse hash
    for _ in 0..64 {
        for input in inputs.iter() {
            let input: u64 = *input as u64;
            // reverse [pos, pos + input]
            let t = hash.clone();
            for i in pos..(pos + input) {
                hash[(i % 256) as usize] = t[((2 * pos + input - 1 - i) % 256) as usize];
            }
            // increase pos
            pos += input + skip;
            skip += 1;
        }
    }

    // Compute dense hash
    let mut hash2 = String::new();
    for chunk in hash.chunks(16) {
        let mut xor = 0;
        for t in chunk.iter() {
            xor = xor ^ t;
        }
        hash2.push_str(&format!("{:02x}", xor));
    }

    return hash2;
}
