use std::collections::HashMap;

pub fn run(input: u64) {
    let part1 = solve_part1(input);
    println!("part1: {}", part1);
    assert_eq!(part1, 371);

    let part2 = solve_part2(input);
    println!("part2: {}", part2);
    assert_eq!(part2, 369601);
}

fn solve_part1(input: u64) -> i64 {
    // for part1, we don't need to store anything. We just "walk" in a spiral, and stop when we
    // reach our input.
    let mut pos: (i64, i64) = (0, 0);
    let mut sq = 1;
    let mut len = 1;

    loop {
        // move right len squares
        let solution = do_move(&mut pos, &mut sq, input, len, (1, 0));
        if solution.is_some() {
            return solution.unwrap();
        }
        // move up len squares
        let solution = do_move(&mut pos, &mut sq, input, len, (0, 1));
        if solution.is_some() {
            return solution.unwrap();
        }
        len += 1;
        // move left len squares
        let solution = do_move(&mut pos, &mut sq, input, len, (-1, 0));
        if solution.is_some() {
            return solution.unwrap();
        }
        // move down len squares
        let solution = do_move(&mut pos, &mut sq, input, len, (0, -1));
        if solution.is_some() {
            return solution.unwrap();
        }
        len += 1;
    }
}

fn do_move(pos: &mut (i64, i64),
           sq: &mut u64,
           input: u64,
           len: u64,
           dir: (i64, i64))
           -> Option<i64> {
    for _ in 0..len {
        *pos = (pos.0 + dir.0, pos.1 + dir.1);
        *sq += 1;
        if *sq == input {
            return Some(pos.0.abs() + pos.1.abs());
        }
    }
    return None;
}

fn solve_part2(input: u64) -> u64 {
    // same trick as in day 3 of 2015: the easiest way to deal with an unknown-sized grid is to
    // use a map.
    let mut grid = HashMap::new();
    let mut pos: (i64, i64) = (0, 0);
    grid.insert(pos, 1);
    let mut len = 1;

    loop {
        // move right len squares
        let solution = do_fill(&mut grid, &mut pos, input, len, (1, 0));
        if solution.is_some() {
            return solution.unwrap();
        }
        // move up len squares
        let solution = do_fill(&mut grid, &mut pos, input, len, (0, 1));
        if solution.is_some() {
            return solution.unwrap();
        }
        len += 1;
        // move left len squares
        let solution = do_fill(&mut grid, &mut pos, input, len, (-1, 0));
        if solution.is_some() {
            return solution.unwrap();
        }
        // move down len squares
        let solution = do_fill(&mut grid, &mut pos, input, len, (0, -1));
        if solution.is_some() {
            return solution.unwrap();
        }
        len += 1;
    }
}

fn do_fill(grid: &mut HashMap<(i64, i64), u64>,
           pos: &mut (i64, i64),
           input: u64,
           len: u64,
           dir: (i64, i64))
           -> Option<u64> {
    for _ in 0..len {
        *pos = (pos.0 + dir.0, pos.1 + dir.1);
        let mut sum = 0;
        for p in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            sum += *grid.entry((pos.0 + p.0, pos.1 + p.1)).or_insert(0);
        }
        grid.insert(*pos, sum);
        if sum > input {
            return Some(sum);
        }
    }
    return None;
}
