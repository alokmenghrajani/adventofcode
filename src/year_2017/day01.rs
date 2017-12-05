pub fn run(input: &[u8]) {
    // duplicate input so we don't have to care about going out of bounds
    let len = input.len();
    let mut buf = Vec::with_capacity(len * 2);
    buf.extend_from_slice(input);
    buf.extend_from_slice(input);

    let part1 = solve_part1(&buf, len);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1044);

    let part2 = solve_part2(&buf, len);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1054);
}

fn solve_part1(buf: &Vec<u8>, len: usize) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..len {
        if buf[i] == buf[i + 1] {
            sum += (buf[i] - '0' as u8) as u64;
        }
    }
    return sum;
}


fn solve_part2(buf: &Vec<u8>, len: usize) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..len {
        if buf[i] == buf[i + len / 2] {
            sum += (buf[i] - '0' as u8) as u64;
        }
    }
    return sum;
}
