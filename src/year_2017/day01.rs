pub fn run(input: &[u8]) {
    // duplicate input so we don't have to care about going out of bounds
    let len = input.len();
    let mut buf = Vec::with_capacity(len * 2);
    buf.extend_from_slice(input);
    buf.extend_from_slice(input);

    part1(&buf, len);
    part2(&buf, len);
}

fn part1(buf: &Vec<u8>, len: usize) {
    let mut sum: u64 = 0;
    for i in 0..len {
        if buf[i] == buf[i + 1] {
            sum += (buf[i] - '0' as u8) as u64;
        }
    }
    println!("part1: {}", sum);
}


fn part2(buf: &Vec<u8>, len: usize) {
    let mut sum: u64 = 0;
    for i in 0..len {
        if buf[i] == buf[i + len / 2] {
            sum += (buf[i] - '0' as u8) as u64;
        }
    }
    println!("part2: {}", sum);
}
