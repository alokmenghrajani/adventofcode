pub fn run(input: Vec<String>) {
    let part1 = solve(&input, 100, false);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1061);

    let part2 = solve(&input, 100, true);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1006);
}

fn solve(input: &Vec<String>, iterations: u64, sticky: bool) -> u64 {
    let mut grid = [[false; 100]; 100];

    // process each line of input
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid[y][x] = true;
            }
        }
    }

    // run Conway's game of life iterations number of times
    for _ in 0..iterations {
        // create a copy of the existing grid
        let mut grid2 = [[false; 100]; 100];
        for y in 0..100 {
            for x in 0..100 {
                grid2[y][x] = grid[y][x];
            }
        }

        for y in 0..100 {
            for x in 0..100 {
                // count neighbors
                let mut c = 0;
                for dy in -1..2 {
                    for dx in -1..2 {
                        if (dx == 0) && (dy == 0) {
                            continue;
                        }
                        if y + dy < 0 {
                            continue;
                        }
                        if y + dy >= 100 {
                            continue;
                        }
                        if x + dx < 0 {
                            continue;
                        }
                        if x + dx >= 100 {
                            continue;
                        }

                        if grid2[(y + dy) as usize][(x + dx) as usize] {
                            c += 1;
                        }
                    }
                }

                // change state on grid but by looking up in grid2
                if grid2[y as usize][x as usize] {
                    grid[y as usize][x as usize] = (c == 2) || (c == 3);
                } else {
                    grid[y as usize][x as usize] = c == 3;
                }
            }
        }

        if sticky {
            grid[0][0] = true;
            grid[99][0] = true;
            grid[0][99] = true;
            grid[99][99] = true;
        }
    }

    // count how many lights are lit
    let mut lit = 0;
    for x in grid.iter() {
        for y in x.iter() {
            if *y {
                lit += 1;
            }
        }
    }
    return lit;
}
