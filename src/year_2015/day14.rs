use regex::Regex;

pub fn run(input: Vec<String>) {
    let part1 = solve(input.clone(), 2503).0;
    println!("part 1: {}", part1);
    assert_eq!(part1, 2660);

    let part2 = solve(input.clone(), 2503).1;
    println!("part 2: {}", part2);
    assert_eq!(part2, 1256);
}

struct State {
    speed: u64,
    flight: u64,
    rest: u64,
    position: u64,
    is_flying: bool,
    countdown: u64,
    points: u64,
}

impl State {
    fn new(speed: u64, flight: u64, rest: u64) -> State {
        return State {
            speed: speed,
            flight: flight,
            rest: rest,
            position: 0,
            is_flying: true,
            countdown: flight,
            points: 0,
        };
    }
}

fn solve(input: Vec<String>, len: u64) -> (u64, u64) {
    // Parse the input. For each reindeer, we want to keep track their
    // speed, flight time, rest time and some state.
    let mut reindeers: Vec<State> = Vec::new();

    let re = Regex::new(r"^.+? can fly (\d+?) km/s for (\d+?) seconds, but then must rest for (\d+?) seconds\.$").unwrap();
    for line in input.iter() {
        match re.captures(line) {
            Some(cap) => {
                let speed: u64 = cap.get(1).unwrap().as_str().parse().unwrap();
                let flight: u64 = cap.get(2).unwrap().as_str().parse().unwrap();
                let rest: u64 = cap.get(3).unwrap().as_str().parse().unwrap();
                reindeers.push(State::new(speed, flight, rest));
            }
            None => {
                panic!("invalid input: {}", line);
            }
        }
    }

    // For part1, we can directly calculate the distance traveled for each reindeer without having
    // to simulate each second. For part2, it's easier to simulate each second, so we do that for
    // both parts.
    for _ in 0..len {
        for reindeer in reindeers.iter_mut() {
            if reindeer.is_flying {
                reindeer.position += reindeer.speed;
            }
            reindeer.countdown -= 1;
            if reindeer.countdown == 0 {
                if reindeer.is_flying {
                    reindeer.is_flying = false;
                    reindeer.countdown = reindeer.rest;
                } else {
                    reindeer.is_flying = true;
                    reindeer.countdown = reindeer.flight;
                }
            }
        }
        let max_position = reindeers.iter().map(|s| s.position).max().unwrap();
        for reindeer in reindeers.iter_mut() {
            if reindeer.position == max_position {
                reindeer.points += 1;
            }
        }
    }

    let max_position = reindeers.iter().map(|s| s.position).max().unwrap();
    let max_points = reindeers.iter().map(|s| s.points).max().unwrap();

    return (max_position, max_points);
}
