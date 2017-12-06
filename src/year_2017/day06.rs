use std::collections::HashMap;
use common::sfy::Sfy;

pub fn run(input: &[u8]) {
    let part1 = solve(input).0;
    println!("part 1: {}", part1);
    assert_eq!(part1, 11137);

    let part2 = solve(input).1;
    println!("part 2: {}", part2);
    assert_eq!(part2, 1037);
}

fn solve(input: &[u8]) -> (u64, u64) {
    // split input on \t
    let mut state: Vec<u8> = input.sfy().split("\t").map(|e| e.parse().unwrap()).collect();

    // track the states we have seen in a HashMap
    let mut seen = HashMap::new();
    let mut steps = 0;
    loop {
        // record that we have seen the current state
        seen.insert(state.clone(), steps);
        // update state
        process(&mut state);
        steps += 1;
        // detect infinite loop
        if seen.contains_key(&state) {
            return (steps, steps - *seen.get(&state).unwrap());
        }
    }
}

fn process(input: &mut [u8]) {
    // find max element.
    let mut max = (0, 0);
    for i in 0..input.len() {
        if input[i] > max.1 {
            max = (i, input[i]);
        }
    }

    // zero the element we found
    input[max.0] = 0;

    // increment by one the blocks which come right after
    for i in 1..(max.1 + 1) {
        let len = input.len();
        input[(max.0 + i as usize) % len] += 1;
    }
}
