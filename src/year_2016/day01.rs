use std;

pub fn run(input: &str) {
    assert_eq!(solve_part1("R2, L3"), 5);
    assert_eq!(solve_part1("R2, R2, R2"), 2);
    assert_eq!(solve_part1("R5, L5, R5, R3"), 12);
    let part1 = solve_part1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 243);

    assert_eq!(solve_part2("R8, R4, R4, R8"), 4);
    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 142);
}

fn solve_part1(input: &str) -> isize {
    _solve(input).0
}

fn solve_part2(input: &str) -> isize {
    _solve(input).1
}

fn _solve(input: &str) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dir: isize = 0;
    let mut visited: Vec<(isize, isize)> = Vec::new();
    let mut first_visited = None;

    for substr in input.trim().split(", ") {
        match substr.chars().next().unwrap() {
            'R' => dir += 1,
            'L' => dir -= 1,
            _ => panic!("expecting R or L"),
        };
        let value: i32 = substr[1..].parse().expect("number must follow R or L");
        for _ in 0..value {
            x += ((dir as f64) * std::f64::consts::PI / 2.0).sin().round() as isize;
            y += ((dir as f64) * std::f64::consts::PI / 2.0).cos().round() as isize;
            if first_visited == None {
                for &(tx, ty) in &visited {
                    if tx == x && ty == y {
                        first_visited = Some(dist(x, y));
                        break;
                    }
                }
            }
            visited.push((x, y));
        }
    }
    match first_visited {
        Some(v) => (dist(x, y), v),
        None => (dist(x, y), -1),
    }
}

fn dist(x: isize, y: isize) -> isize {
    x.abs() + y.abs()
}
