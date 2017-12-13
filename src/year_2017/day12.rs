use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    println!("pro-tip: compile with --release!");

    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 130);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 189);
}

fn solve_part1(input: &Vec<String>) -> usize {
    let minimal_sets = common(input);
    for set in minimal_sets.iter() {
        if set.contains(&0) {
            return set.len();
        }
    }
    return 0;
}

fn solve_part2(input: &Vec<String>) -> usize {
    let minimal_sets = common(input);
    return minimal_sets.len();
}

fn common(input: &Vec<String>) -> Vec<HashSet<u64>> {
    // Parse the input and create an initial list of sets.
    let mut sets: Vec<HashSet<u64>> = Vec::new();
    for line in input.iter() {
        let mut t = line.split(" <-> ");
        let mut set: HashSet<u64> = HashSet::new();
        set.insert(t.next().unwrap().parse().unwrap());
        for i in t.next().unwrap().split(", ") {
            set.insert(i.parse().unwrap());
        }
        sets.push(set);
    }

    // Build a new line of sets, which is a minimal list:
    // - pick a random set and merge as many of the other sets as possible.
    // - store the super-set in a new vector.
    let mut minimal_sets: Vec<HashSet<u64>> = Vec::new();
    while sets.len() > 0 {
        let mut set1 = sets.pop().unwrap();
        let mut i = 0;
        while i < sets.len() {
            let mut merged = false;
            {
                // Had to fight the borrow checker a little on this one.
                let set2 = &sets[i];
                if !set1.is_disjoint(set2) {
                    merged = true;
                    for s in set2.iter() {
                        set1.insert(*s);
                    }
                }
            }
            if merged {
                sets.swap_remove(i);
                // Re-start the loop if we merged anything. There's probably more efficient ways
                // to build minimal_sets.
                i = 0;
            } else {
                i += 1;
            }
        }
        minimal_sets.push(set1);
    }
    return minimal_sets;
}
