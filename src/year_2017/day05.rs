pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 355965);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 26948068);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    // convert the Vec<String> to Vec<i64>
    let mut program: Vec<i64> = input.iter().map(|s| s.parse().unwrap()).collect();
    let mut pc: i64 = 0;
    let mut steps = 0;
    loop {
        // check if we are out of bounds
        if (pc < 0) || (pc >= program.len() as i64) {
            return steps;
        }
        // store the current jump offset in a temp variable
        let t = program[pc as usize];
        // update the program
        program[pc as usize] += 1;
        // update the program counter and step counter
        pc += t;
        steps += 1;
    }
}

fn solve_part2(input: &Vec<String>) -> u64 {
    // convert the Vec<String> to Vec<i64>
    let mut program: Vec<i64> = input.iter().map(|s| s.parse().unwrap()).collect();
    let mut pc: i64 = 0;
    let mut steps = 0;
    loop {
        // check if we are out of bounds
        if (pc < 0) || (pc >= program.len() as i64) {
            return steps;
        }
        // store the current jump offset in a temp variable
        let t = program[pc as usize];
        // update the program
        if t >= 3 {
            program[pc as usize] -= 1;
        } else {
            program[pc as usize] += 1;
        }
        // update the program counter and step counter
        pc += t;
        steps += 1;
    }
}
