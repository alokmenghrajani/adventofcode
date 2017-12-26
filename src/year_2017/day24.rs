use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(input.clone());
    println!("part 1: {}", part1);
    assert_eq!(part1, 1656);

    let part2 = solve_part2(input.clone());
    println!("part 2: {}", part2);
    assert_eq!(part2, 1642);
}

fn solve_part1(input: Vec<String>) -> u64 {
    // Parse the input
    let pieces = parse(input);

    // recursively explore the search space.
    return solve_part1_recursive(&pieces, 0, HashSet::new());
}

fn solve_part2(input: Vec<String>) -> u64 {
    // Parse the input
    let pieces = parse(input);

    // recursively explore the search space.
    return solve_part2_recursive(&pieces, 0, HashSet::new()).1;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Piece {
    id: usize,
    port1: u64,
    port2: u64,
}

fn solve_part1_recursive(pieces: &HashMap<u64, Vec<Piece>>,
                         current: u64,
                         visited: HashSet<usize>)
                         -> u64 {
    let empty = Vec::new();
    let v = pieces.get(&current).unwrap_or(&empty);
    return v.iter()
        .filter(|piece| !visited.contains(&piece.id))
        .map(|piece| {
            let piece = piece.clone();
            let mut visited = visited.clone();
            visited.insert(piece.id);
            let t: u64 = if piece.port1 == current {
                current * 2 + solve_part1_recursive(&pieces, piece.port2, visited)
            } else {
                assert_eq!(piece.port2, current);
                current * 2 + solve_part1_recursive(&pieces, piece.port1, visited)
            };
            return t;
        })
        .max()
        .unwrap_or(current);
}

fn solve_part2_recursive(pieces: &HashMap<u64, Vec<Piece>>,
                         current: u64,
                         visited: HashSet<usize>)
                         -> (u64, u64) {
    let empty = Vec::new();
    let v = pieces.get(&current).unwrap_or(&empty);
    return v.iter()
        .filter(|piece| !visited.contains(&piece.id))
        .map(|piece| {
            let piece = piece.clone();
            let mut visited = visited.clone();
            visited.insert(piece.id);
            let depth: u64;
            let strength: u64;
            if piece.port1 == current {
                let t = solve_part2_recursive(&pieces, piece.port2, visited);
                depth = t.0 + 1;
                strength = t.1 + current * 2;
            } else {
                assert_eq!(piece.port2, current);
                let t = solve_part2_recursive(&pieces, piece.port1, visited);
                depth = t.0 + 1;
                strength = t.1 + current * 2;
            };
            return (depth, strength);
        })
        .max_by(|&(depth1, strength1), &(depth2, strength2)| if depth1 == depth2 {
            return strength1.cmp(&strength2);
        } else {
            return depth1.cmp(&depth2);
        })
        .unwrap_or((0, current));
}

fn parse(input: Vec<String>) -> HashMap<u64, Vec<Piece>> {
    let mut m = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        let mut t = line.split('/');
        let p = Piece {
            id: i,
            port1: t.next().unwrap().parse().unwrap(),
            port2: t.next().unwrap().parse().unwrap(),
        };
        {
            let v = m.entry(p.port1).or_insert(Vec::new());
            v.push(p.clone());
        }
        {
            let v = m.entry(p.port2).or_insert(Vec::new());
            v.push(p.clone());
        }
    }
    return m;
}
