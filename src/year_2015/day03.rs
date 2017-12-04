use std::collections::HashMap;

pub fn run(input: &[u8]) {
    let part1 = solve_part1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 2565);

    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 2639);
}

fn solve_part1(buf: &[u8]) -> i64 {
    // the easiest way to deal with an unknown-sized grid is to use a map.
    let mut grid = HashMap::new();
    let mut pos = (0, 0);
    let mut sum = 0;

    // deliver present at the first house
    grid.insert(pos, true);
    sum += 1;

    // loop over every character
    for i in 0..buf.len() {
        // update pos
        match buf[i] {
            b'^' => pos = (pos.0, pos.1 - 1),
            b'>' => pos = (pos.0 + 1, pos.1),
            b'<' => pos = (pos.0 - 1, pos.1),
            b'v' => pos = (pos.0, pos.1 + 1),
            _ => panic!("invalid input: {}", buf[i]),
        }

        // if the grid at the new position is empty, increment sum
        grid.entry(pos).or_insert_with(|| {
            sum += 1;
            return true;
        });
    }
    return sum;
}

fn solve_part2(buf: &[u8]) -> i64 {
    // the easiest way to deal with an unknown-sized grid is to use a map.
    let mut grid = HashMap::new();
    let mut pos = [(0, 0), (0, 0)];
    let mut sum = 0;

    // deliver present at the first house
    grid.insert(pos[0], true);
    sum += 1;

    // loop over every character
    for i in 0..buf.len() {
        match buf[i] {
            b'^' => pos[i % 2] = (pos[i % 2].0, pos[i % 2].1 - 1),
            b'>' => pos[i % 2] = (pos[i % 2].0 + 1, pos[i % 2].1),
            b'<' => pos[i % 2] = (pos[i % 2].0 - 1, pos[i % 2].1),
            b'v' => pos[i % 2] = (pos[i % 2].0, pos[i % 2].1 + 1),
            _ => panic!("invalid input: {}", buf[i]),
        }
        grid.entry(pos[i % 2]).or_insert_with(|| {
            sum += 1;
            return true;
        });
    }
    return sum;
}
