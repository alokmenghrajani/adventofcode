use regex::Regex;
use std::collections::HashMap;

pub fn run(input: Vec<String>) {
    let part1 = solve(input.clone(), 5);
    println!("part 1: {}", part1);
    assert_eq!(part1, 164);

    let part2 = solve(input.clone(), 18);
    println!("part 2: {}", part2);
    assert_eq!(part2, 2355110);
}

fn solve(input: Vec<String>, iter: u64) -> usize {
    // Parse the input.
    let mut rules: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();

    let re1 = Regex::new(r"^(..)/(..) => (...)/(...)/(...)$").unwrap();
    let re2 = Regex::new(r"^(...)/(...)/(...) => (....)/(....)/(....)/(....)$").unwrap();
    for line in input.iter() {
        match re1.captures(line) {
            Some(cap) => {
                let inp1: Vec<char> = cap.get(1).unwrap().as_str().chars().collect();
                let inp2: Vec<char> = cap.get(2).unwrap().as_str().chars().collect();
                let out1: Vec<char> = cap.get(3).unwrap().as_str().chars().collect();
                let out2: Vec<char> = cap.get(4).unwrap().as_str().chars().collect();
                let out3: Vec<char> = cap.get(5).unwrap().as_str().chars().collect();
                let inp = vec![inp1, inp2];
                let out = vec![out1, out2, out3];

                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());

                let inp = flip(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());

                continue;
            }
            None => {}
        }
        match re2.captures(line) {
            Some(cap) => {
                let inp1: Vec<char> = cap.get(1).unwrap().as_str().chars().collect();
                let inp2: Vec<char> = cap.get(2).unwrap().as_str().chars().collect();
                let inp3: Vec<char> = cap.get(3).unwrap().as_str().chars().collect();
                let out1: Vec<char> = cap.get(4).unwrap().as_str().chars().collect();
                let out2: Vec<char> = cap.get(5).unwrap().as_str().chars().collect();
                let out3: Vec<char> = cap.get(6).unwrap().as_str().chars().collect();
                let out4: Vec<char> = cap.get(7).unwrap().as_str().chars().collect();
                let inp = vec![inp1, inp2, inp3];
                let out = vec![out1, out2, out3, out4];

                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());

                let inp = flip(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());
                let inp = rotate(inp);
                rules.insert(inp.clone(), out.clone());

                continue;
            }
            None => {}
        }
        panic!("failed to parse: {}", line);
    }

    let mut size = 3;
    let mut grid = new_grid(size);
    grid[0][1] = '#';
    grid[1][2] = '#';
    grid[2][0] = '#';
    grid[2][1] = '#';
    grid[2][2] = '#';

    for _ in 0..iter {
        if size % 2 == 0 {
            let mut grid2 = new_grid(size / 2 * 3);
            for i in 0..(size / 2) {
                for j in 0..(size / 2) {
                    // extract grid[i*2..i*2+2][j*2..j*2+2]
                    let mut t = new_grid(2);
                    for ii in 0..2 {
                        for jj in 0..2 {
                            t[ii][jj] = grid[i * 2 + ii][j * 2 + jj];
                        }
                    }

                    // find the transform
                    let rule = rules.get(&t).unwrap();

                    // fill the new grid
                    for ii in 0..3 {
                        for jj in 0..3 {
                            grid2[i * 3 + ii][j * 3 + jj] = rule[ii][jj];
                        }
                    }
                }
            }
            size = size / 2 * 3;
            grid = grid2;
        } else {
            let mut grid2 = new_grid(size / 3 * 4);
            for i in 0..(size / 3) {
                for j in 0..(size / 3) {
                    // extract grid[i*3..i*3+3][j*3..j*3+3]
                    let mut t = new_grid(3);
                    for ii in 0..3 {
                        for jj in 0..3 {
                            t[ii][jj] = grid[i * 3 + ii][j * 3 + jj];
                        }
                    }

                    // find the transform
                    let rule = rules.get(&t).unwrap();

                    // fill the new grid
                    for ii in 0..4 {
                        for jj in 0..4 {
                            grid2[i * 4 + ii][j * 4 + jj] = rule[ii][jj];
                        }
                    }
                }
            }
            size = size / 3 * 4;
            grid = grid2;
        }
    }

    // count number of cells which are on
    let mut c = 0;
    for i in 0..size {
        for j in 0..size {
            if grid[i][j] == '#' {
                c += 1;
            }
        }
    }

    return c;
}

fn new_grid(size: usize) -> Vec<Vec<char>> {
    let mut r = Vec::new();
    for _ in 0..size {
        let mut t = Vec::new();
        for _ in 0..size {
            t.push('.');
        }
        r.push(t);
    }
    return r;
}

fn rotate(inp: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let l = inp.len();
    let mut r = new_grid(l);

    for i in 0..l {
        for j in 0..l {
            r[l - j - 1][i] = inp[i][j];
        }
    }

    return r;
}

fn flip(inp: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let l = inp.len();
    let mut r = new_grid(l);

    for i in 0..l {
        for j in 0..l {
            r[l - i - 1][j] = inp[i][j];
        }
    }

    return r;
}
