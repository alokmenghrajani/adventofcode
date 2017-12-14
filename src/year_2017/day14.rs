use year_2017::day10::knot_hash;
use std::collections::HashMap;

enum Cell {
    Used,
    Colored,
}

pub fn run(input: &str) {
    let part1 = solve_part1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 8226);

    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1128);
}

fn solve_part1(input: &str) -> usize {
    // convert the input into a grid
    let grid = build_grid(input);
    let mut used = 0;

    return grid.len();
}

fn solve_part2(input: &str) -> u64 {
    // convert the input into a grid
    let mut grid = build_grid(input);
    let mut colors = 0;

    // until all cells have been colored,
    // find the first cell which has not yet been "colored",
    // and color it.
    loop {
        let mut found = None;
        {
            for cell in grid.iter() {
                match cell {
                    (&pos, &Cell::Used) => {
                        found = Some(pos);
                        break;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        match found {
            Some(pos) => {
                colors += 1;
                flood(&mut grid, pos);
            }
            None => {
                // we are done!
                break;
            }
        }
    }

    return colors;
}

fn flood(grid: &mut HashMap<(isize, isize), Cell>, start: (isize, isize)) {
    let mut cells = vec![start];
    while !cells.is_empty() {
        // as long as we have connected cells, pick the first one
        // and color it.
        let cell = cells.pop().unwrap();
        match grid.get(&cell) {
            None => panic!("free cell in flood"),
            Some(&Cell::Colored) => continue,
            Some(_) => {}
        }

        grid.insert(cell, Cell::Colored);

        // look at neighbors and add them to the list of cells to color
        // if they haven't been colored.
        for d in [(1, 0), (-1, 0), (0, -1), (0, 1)].iter() {
            let pos = (cell.0 + d.0, cell.1 + d.1);
            match grid.get(&pos) {
                Some(&Cell::Used) => cells.push(pos),
                _ => {}
            }
        }
    }
}

fn build_grid(input: &str) -> HashMap<(isize, isize), Cell> {
    let mut grid: HashMap<(isize, isize), Cell> = HashMap::new();
    for i in 0..128 {
        let hash_input = format!("{}-{}", input, i);
        let hash_output = knot_hash(hash_input.as_bytes());
        for j in 0..32 {
            let hex = hash_output.as_bytes()[j as usize];
            let val = match b"0123456789abcdef".iter().position(|&c| c == hex) {
                Some(p) => p,
                None => panic!("invalid hex char: {}", hex),
            };
            for k in 0..4 {
                if (val >> (3 - k)) & 1 == 1 {
                    grid.insert((i, j * 4 + k), Cell::Used);
                }
            }
        }
    }
    return grid;
}
