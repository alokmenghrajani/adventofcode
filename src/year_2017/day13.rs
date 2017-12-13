pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1904);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 3833504);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    // Parse the input and store the data in a vec
    let mut firewall: Vec<u64> = Vec::with_capacity(0);
    let mut max = 0;
    for line in input.iter() {
        let mut t = line.split(": ");
        let k: usize = t.next().unwrap().parse().unwrap();
        if k >= firewall.capacity() {
            let delta = firewall.capacity() - k + 1;
            firewall.reserve(delta);
            for _ in 0..(firewall.capacity() - firewall.len()) {
                firewall.push(0);
            }
        }
        if k > max {
            max = k;
        }
        let v = t.next().unwrap().parse().unwrap();
        firewall[k] = v;
    }

    // Iterate over the layers and compute the score.
    let mut score = 0;
    for p in 0..(max + 1) {
        if firewall[p] == 0 {
            // no scanner
            continue;
        }
        if (p as u64) % (2 * firewall[p] - 2) == 0 {
            // we got caught
            score += (p as u64) * firewall[p];
        }
    }
    return score;
}

fn solve_part2(input: &Vec<String>) -> u64 {
    // Parse the input and store the data in a vec
    let mut firewall: Vec<u64> = Vec::with_capacity(0);
    let mut max = 0;
    for line in input.iter() {
        let mut t = line.split(": ");
        let k: usize = t.next().unwrap().parse().unwrap();
        if k >= firewall.capacity() {
            let delta = firewall.capacity() - k + 1;
            firewall.reserve(delta);
            for _ in 0..(firewall.capacity() - firewall.len()) {
                firewall.push(0);
            }
        }
        if k > max {
            max = k;
        }
        let v = t.next().unwrap().parse().unwrap();
        firewall[k] = v;
    }

    let mut delay = 0;
    'outer: loop {
        for p in 0..(max + 1) {
            if firewall[p] == 0 {
                // no scanner
                continue;
            }
            if (delay + p as u64) % (2 * firewall[p] - 2) == 0 {
                delay += 1;
                continue 'outer;
            }
        }
        return delay;
    }
}
