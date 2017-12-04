use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn run(input: &str) {
    println!("pro-tip: compile with --release!");
    let part1 = solve(input.as_bytes(), 5);
    println!("part 1: {}", part1);
    assert_eq!(part1, 254575);

    let part2 = solve(input.as_bytes(), 6);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1038736);
}

fn solve(prefix: &[u8], num_zeros: usize) -> u64 {
    let mut i = 0;
    let mut buf = Vec::new();
    buf.extend_from_slice(prefix);
    let mut md5 = Md5::new();
    let desired_prefix = "0".repeat(num_zeros);
    loop {
        buf.truncate(prefix.len());
        buf.extend_from_slice(format!("{}", i).as_bytes());
        md5.reset();

        // check md5
        md5.input(&buf);
        let res = md5.result_str();
        if res.starts_with(&desired_prefix) {
            return i;
        }
        i += 1;
    }
}
