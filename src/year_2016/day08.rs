extern crate regex;
use self::regex::Regex;

pub fn solve(input: &str) {
    let mut test_case = Rect::new(7, 3);
    test_case.rect(3, 2);
    assert_eq!(test_case.to_string(), "###....\n###....\n.......\n");

    test_case.rotate_column(1, 1);
    assert_eq!(test_case.to_string(), "#.#....\n###....\n.#.....\n");

    test_case.rotate_row(0, 4);
    assert_eq!(test_case.to_string(), "....#.#\n###....\n.#.....\n");

    test_case.rotate_column(1, 1);
    assert_eq!(test_case.to_string(), ".#..#.#\n#.#....\n.#.....\n");

    println!("part 1: {}", part1(input));
    println!("part 2:\n{}", part2(input));
}

fn part1(input: &str) -> usize {
    _solve(input).chars().map(|x| if x == '#' { 1 } else { 0 }).sum()
}

fn part2(input: &str) -> String {
    _solve(input)
}

fn _solve(input: &str) -> String {
    let mut screen = Rect::new(50, 6);
    let rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let rotate_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let rotate_column = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    for line in input.trim().split("\n") {
        if let Some(cap) = rect.captures(line) {
            screen.rect(cap.at(1).unwrap().parse().unwrap(),
                        cap.at(2).unwrap().parse().unwrap());
        } else if let Some(cap) = rotate_row.captures(line) {
            screen.rotate_row(cap.at(1).unwrap().parse().unwrap(),
                              cap.at(2).unwrap().parse().unwrap());
        } else if let Some(cap) = rotate_column.captures(line) {
            screen.rotate_column(cap.at(1).unwrap().parse().unwrap(),
                                 cap.at(2).unwrap().parse().unwrap());
        } else {
            panic!("unexpected input");
        }
    }
    screen.to_string_nice(true)
}

struct Rect {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

impl Rect {
    fn new(width: usize, height: usize) -> Rect {
        let mut p = Vec::with_capacity(width);
        for i in 0..width {
            p.push(Vec::with_capacity(height));
            for _ in 0..height {
                p[i].push(false);
            }
        }
        Rect {
            width: width,
            height: height,
            pixels: p,
        }
    }

    fn rect(&mut self, w: usize, h: usize) {
        for i in 0..w {
            for j in 0..h {
                self.pixels[i][j] = true;
            }
        }
    }

    fn transform<F>(&mut self, f: F)
        where F: Fn(usize, usize) -> (isize, isize)
    {
        let mut p = Vec::with_capacity(self.width);
        for i in 0..self.width {
            p.push(Vec::with_capacity(self.height));
            for j in 0..self.height {
                let (mut new_i, mut new_j) = f(i, j);
                new_i = (new_i + self.width as isize) % self.width as isize;
                new_j = (new_j + self.height as isize) % self.height as isize;
                p[i].push(self.pixels[new_i as usize][new_j as usize]);
            }
        }
        self.pixels = p;
    }

    fn rotate_column(&mut self, x: usize, amount: isize) {
        self.transform(|i, j| if i == x {
            (i as isize, j as isize - amount)
        } else {
            (i as isize, j as isize)
        });
    }

    fn rotate_row(&mut self, y: usize, amount: isize) {
        self.transform(|i, j| if j == y {
            (i as isize - amount, j as isize)
        } else {
            (i as isize, j as isize)
        });
    }

    fn to_string(&self) -> String {
        self.to_string_nice(false)
    }

    fn to_string_nice(&self, nice: bool) -> String {
        let mut r = String::new();
        for j in 0..self.height {
            for i in 0..self.width {
                if nice && i % 5 == 0 {
                    r.push_str("  ")
                }
                if self.pixels[i][j] {
                    r.push_str("#")
                } else {
                    r.push_str(if nice { " " } else { "." })
                }
            }
            r.push('\n')
        }
        r
    }
}
