use regex::Regex;
use regex::Captures;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 569999);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 17836115);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    let mut grid = [[false; 1000]; 1000];

    let re = Regex::new(r"^(.+) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    // process each line of input
    for line in input.iter() {
        // parse line.
        let cap = re.captures(line);
        if cap.is_none() {
            panic!("invalid input: {}", line);
        }
        let cap = cap.unwrap();
        match cap.get(1).unwrap().as_str() {
            "turn on" => process(&mut grid, cap, |_| true),
            "toggle" => process(&mut grid, cap, |e| !e),
            "turn off" => process(&mut grid, cap, |_| false),
            _ => panic!("invalid input: {}", line),
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

fn solve_part2(input: &Vec<String>) -> u64 {
    let mut grid = [[0; 1000]; 1000];

    let re = Regex::new(r"^(.+) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    // process each line of input
    for line in input.iter() {
        // parse line.
        let cap = re.captures(line).expect(&format!("invalid input: {}", line));
        match cap.get(1).unwrap().as_str() {
            "turn on" => process(&mut grid, cap, |e| e + 1),
            "toggle" => process(&mut grid, cap, |e| e + 2),
            "turn off" => process(&mut grid, cap, |e| if e > 0 { e - 1 } else { 0 }),
            _ => panic!("invalid input: {}", line),
        }
    }

    // total brightness
    let mut brightness = 0;
    for x in grid.iter() {
        for y in x.iter() {
            brightness += *y;
        }
    }
    return brightness;
}

fn process<T, F>(grid: &mut [[T; 1000]; 1000], cap: Captures, f: F)
    where F: Fn(T) -> T,
          T: Copy
{
    let from_x: usize = cap.get(2).unwrap().as_str().parse().unwrap();
    let from_y: usize = cap.get(3).unwrap().as_str().parse().unwrap();
    let to_x: usize = cap.get(4).unwrap().as_str().parse().unwrap();
    let to_y: usize = cap.get(5).unwrap().as_str().parse().unwrap();

    for x in from_x..(to_x + 1) {
        for y in from_y..(to_y + 1) {
            grid[x][y] = f(grid[x][y]);
        }
    }
}
