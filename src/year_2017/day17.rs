pub fn run(step: usize) {
    let part1 = solve_part1(step);
    println!("part 1: {}", part1);
    assert_eq!(part1, 2000);

    let part2 = solve_part2(step);
    println!("part 2: {}", part2);
    assert_eq!(part2, 10242889);
}

fn solve_part1(step: usize) -> usize {
    // I wasn't able to find a way to re-use code between part1 and part2.
    // For part1, we use a buffer.
    let mut buf = Vec::new();
    buf.push(0);
    let mut pos = 0;
    for i in 1..2018 {
        pos = (pos + step) % i;
        buf.insert(pos, i);
        pos += 1;
    }
    pos = pos % 2017;
    return buf[pos];
}

fn solve_part2(step: usize) -> usize {
    // 0 is at offset 0. It makes the search easy. We no longer need a buffer, just keep track
    // of what was the last element to get added right after 0.
    let mut pos = 0;
    let mut last = None;
    for i in 1..(50_000_000 + 1) {
        pos = (pos + step) % i;
        if pos == 0 {
            last = Some(i);
        }
        pos += 1;
    }
    return last.unwrap();
}
