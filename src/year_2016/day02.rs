use common::sfy::Sfy;

pub fn run(input: &str) {
    assert_eq!(solve_part1("ULL\nRRDDD\nLURDL\nUUUUD"), "1985");
    let part1 = solve_part1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, "82958");

    assert_eq!(solve_part2("ULL\nRRDDD\nLURDL\nUUUUD"), "5DB3");
    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, "B3DB8");
}

fn solve_part1(input: &str) -> String {
    // I found a smart way to solve today's puzzle. Instead of doing bound checking in each
    // direction, I make the keypad a little bigger and put a placeholder '-' character.
    _solve(input,
           (2, 2),
           &[&['-', '-', '-', '-', '-'],
             &['-', '1', '2', '3', '-'],
             &['-', '4', '5', '6', '-'],
             &['-', '7', '8', '9', '-'],
             &['-', '-', '-', '-', '-']])
}

fn solve_part2(input: &str) -> String {
    _solve(input,
           (1, 3),
           &[&['-', '-', '-', '-', '-', '-', '-'],
             &['-', '-', '-', '1', '-', '-', '-'],
             &['-', '-', '2', '3', '4', '-', '-'],
             &['-', '5', '6', '7', '8', '9', '-'],
             &['-', '-', 'A', 'B', 'C', '-', '-'],
             &['-', '-', '-', 'D', '-', '-', '-'],
             &['-', '-', '-', '-', '-', '-', '-']])
}

fn _solve(input: &str, (start_x, start_y): (usize, usize), keypad: &[&[char]]) -> String {
    let mut x = start_x;
    let mut y = start_y;
    let mut r = vec![];
    for line in input.trim().split("\n") {
        for token in line.chars() {
            let (ny, nx) = match token {
                'R' => (y, x + 1),
                'L' => (y, x - 1),
                'D' => (y + 1, x),
                'U' => (y - 1, x),
                _ => panic!(),
            };
            // update the current position if we aren't out of bounds.
            if keypad[ny][nx] != '-' {
                x = nx;
                y = ny;
            }
        }
        // at the end of each line, record the keypad value.
        r.push(keypad[y][x]);
    }
    return r.sfy();
}
