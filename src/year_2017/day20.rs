use regex::Regex;
use std::collections::HashMap;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(input.clone());
    println!("part 1: {}", part1);
    assert_eq!(part1, 308);

    let part2 = solve_part2(input.clone());
    println!("part 2: {}", part2);
    assert_eq!(part2, 504);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
    collided: bool,
}

impl Particle {
    fn new(p: (i64, i64, i64), v: (i64, i64, i64), a: (i64, i64, i64)) -> Particle {
        return Particle {
            p: p,
            v: v,
            a: a,
            collided: false,
        };
    }
}

fn solve_part1(input: Vec<String>) -> usize {
    // Parse the input.
    let mut particles = parse(input);

    // We could try to do smart things, but it's easier to just simulate things for a large amount
    // of ticks.
    for _ in 0..100_000 {
        for particle in particles.iter_mut() {
            particle.v.0 += particle.a.0;
            particle.v.1 += particle.a.1;
            particle.v.2 += particle.a.2;

            particle.p.0 += particle.v.0;
            particle.p.1 += particle.v.1;
            particle.p.2 += particle.v.2;
        }
    }

    // Find the particle which is closest to 0,0,0
    return particles.iter()
        .enumerate()
        .min_by_key(|&(_, p)| p.p.0.abs() + p.p.1.abs() + p.p.2.abs())
        .unwrap()
        .0;
}

fn solve_part2(input: Vec<String>) -> usize {
    // Parse the input.
    let mut particles = parse(input);

    // Same technique as part1. We could try to do smart things, but it's easier to just
    // simulate things for a large amount of ticks.
    for _ in 0..100_000 {
        let mut collisions: HashMap<(i64, i64, i64), usize> = HashMap::new();
        for particle in particles.iter_mut() {
            if particle.collided {
                continue;
            }

            particle.v.0 += particle.a.0;
            particle.v.1 += particle.a.1;
            particle.v.2 += particle.a.2;

            particle.p.0 += particle.v.0;
            particle.p.1 += particle.v.1;
            particle.p.2 += particle.v.2;

            let n = collisions.entry(particle.p).or_insert(0);
            *n += 1;
        }

        for particle in particles.iter_mut() {
            if particle.collided {
                continue;
            }
            if *collisions.get(&particle.p).unwrap() >= 2 {
                particle.collided = true;
            }
        }
    }

    return particles.iter().filter(|p| !p.collided).count();
}


fn parse(input: Vec<String>) -> Vec<Particle> {
    let mut r = Vec::new();

    let re = Regex::new(r"^p=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, v=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, a=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>$").unwrap();
    for line in input.iter() {
        match re.captures(line) {
            Some(cap) => {
                let px = cap.get(1).unwrap().as_str().parse().unwrap();
                let py = cap.get(2).unwrap().as_str().parse().unwrap();
                let pz = cap.get(3).unwrap().as_str().parse().unwrap();

                let vx = cap.get(4).unwrap().as_str().parse().unwrap();
                let vy = cap.get(5).unwrap().as_str().parse().unwrap();
                let vz = cap.get(6).unwrap().as_str().parse().unwrap();

                let ax = cap.get(7).unwrap().as_str().parse().unwrap();
                let ay = cap.get(8).unwrap().as_str().parse().unwrap();
                let az = cap.get(9).unwrap().as_str().parse().unwrap();
                r.push(Particle::new((px, py, pz), (vx, vy, vz), (ax, ay, az)));
            }
            None => panic!("failed to parse: {}", line),
        }
    }

    return r;
}
