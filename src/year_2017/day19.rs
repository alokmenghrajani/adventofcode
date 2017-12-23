use std::collections::HashMap;

pub fn run(input: Vec<String>) {
    let part1 = solve(input.clone()).0;
    println!("part 1: {}", part1);
    assert_eq!(part1, "RYLONKEWB");

    let part2 = solve(input.clone()).1;
    println!("part 2: {}", part2);
    assert_eq!(part2, 16016);
}

fn solve(input: Vec<String>) -> (String, u64) {
    let mut grid = HashMap::new();

    // Parse the input
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            grid.insert((x as i64, y as i64), input[y].chars().nth(x).unwrap());
        }
    }

    // find the starting position
    let mut pos_y: i64 = 0;
    let mut pos_x: i64 = input[0].chars().enumerate().find(|&(_, c)| c == '|').unwrap().0 as i64;
    let mut dir = 0;
    let dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    // move around the grid until we are done
    let mut letters = String::new();
    let mut steps = 0;
    loop {
        let c = *grid.entry((pos_x, pos_y)).or_insert(' ');
        match c {
            'A'...'Z' => {
                letters.push(c);
            }
            '|' | '-' => {}
            '+' => {
                // we either need to go dir+1 or dir-1
                let c1 = *grid.entry((pos_x + dirs[(dir + 1) % 4].0, pos_y + dirs[(dir + 1) % 4].1))
                    .or_insert(' ');
                let c2 = *grid.entry((pos_x + dirs[(dir + 3) % 4].0, pos_y + dirs[(dir + 3) % 4].1))
                    .or_insert(' ');
                match (c1, c2) {
                    (' ', ' ') => panic!("both directions result in space"),
                    (_, ' ') => {
                        dir = (dir + 1) % 4;
                    }
                    (' ', _) => {
                        dir = (dir + 3) % 4;
                    }
                    _ => panic!("neither direction results in a space"),
                }
            }
            ' ' => {
                // we are done
                break;
            }
            _ => panic!("unknown char: {}", c as u8),
        }
        pos_x += dirs[dir].0;
        pos_y += dirs[dir].1;
        steps += 1;
    }

    return (letters, steps);
}
