use common::sfy::Sfy;

pub fn run(input: &[u8]) {
    let part1 = solve(input).0;
    println!("part 1: {}", part1);
    assert_eq!(part1, 743);

    let part2 = solve(input).1;
    println!("part 2: {}", part2);
    assert_eq!(part2, 1493);
}

fn solve(input: &[u8]) -> (i64, i64) {
    // Use cubic coordinates. See https://www.redblobgames.com/grids/hexagons/.
    let mut pos: (i64, i64, i64) = (0, 0, 0);
    let s = input.sfy();
    let moves: Vec<&str> = s.split(",").collect();
    let mut max = 0;
    for m in moves.into_iter() {
        let m = match m {
            "n" => (1, -1, 0),
            "s" => (-1, 1, 0),
            "ne" => (1, 0, -1),
            "sw" => (-1, 0, 1),
            "nw" => (0, 1, -1),
            "se" => (0, -1, 1),
            _ => panic!("invalid input: {}", m),
        };
        pos = (pos.0 + m.0, pos.1 + m.1, pos.2 + m.2);
        let t = (pos.0.abs() + pos.1.abs() + pos.2.abs()) / 2;
        if t > max {
            max = t;
        }
    }
    return ((pos.0.abs() + pos.1.abs() + pos.2.abs()) / 2, max);
}
