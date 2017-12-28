use std::collections::HashMap;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(input.clone(), 150);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1638);

    let part2 = solve_part2(input.clone(), 150);
    println!("part 2: {}", part2);
    assert_eq!(part2, 17);
}

fn solve_part1(input: Vec<String>, eggnog: i64) -> i64 {
    let containers: Vec<i64> = input.iter().map(|line| line.parse().unwrap()).collect();

    // recursively look for a solution.
    // note: it's possible sorting the containers from largest to smallest will make
    // the recursive search go quicker.
    let mut results: HashMap<u64, i64> = HashMap::new();
    solve_rec(&containers, 0, eggnog, 0, &mut results);
    return results.values().sum();
}

fn solve_part2(input: Vec<String>, eggnog: i64) -> i64 {
    let containers: Vec<i64> = input.iter().map(|line| line.parse().unwrap()).collect();

    // recursively look for a solution.
    // note: it's possible sorting the containers from largest to smallest will make
    // the recursive search go quicker.
    let mut results: HashMap<u64, i64> = HashMap::new();
    solve_rec(&containers, 0, eggnog, 0, &mut results);

    let min = results.keys().min().unwrap();
    return *results.get(&min).unwrap();
}

fn solve_rec(containers: &Vec<i64>,
             offset: usize,
             eggnog: i64,
             taken: u64,
             results: &mut HashMap<u64, i64>) {
    if eggnog < 0 {
        return;
    }
    if offset == containers.len() {
        if eggnog == 0 {
            *results.entry(taken).or_insert(0) += 1;
        }
        return;
    }

    // we can either take or skip this container
    solve_rec(containers,
              offset + 1,
              eggnog - containers[offset],
              taken + 1,
              results);
    solve_rec(containers, offset + 1, eggnog, taken, results);
}
