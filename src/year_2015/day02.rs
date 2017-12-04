pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1588178);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 3783758);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    for line in input.iter() {
        // split on 'x'
        let pieces: Vec<&str> = line.split('x').collect();
        // and convert each line into a vector of u64.
        let mut pieces: Vec<u64> = pieces.iter().map(|s| s.parse().unwrap()).collect();
        pieces.sort();

        // compute 2*l*w + 2*w*h + 2*h*l + add the smallest side
        assert_eq!(pieces.len(), 3);
        sum += 2 * pieces[0] * pieces[1] + 2 * pieces[0] * pieces[2] + 2 * pieces[1] * pieces[2] +
               pieces[0] * pieces[1];
    }
    return sum;
}

fn solve_part2(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    for line in input.iter() {
        // split on 'x'
        let pieces: Vec<&str> = line.split('x').collect();
        // and convert each line into a vector of u64.
        let mut pieces: Vec<u64> = pieces.iter().map(|s| s.parse().unwrap()).collect();
        pieces.sort();

        // compute 2*l*w + 2*w*h + 2*h*l + add the smallest side
        assert_eq!(pieces.len(), 3);
        sum += 2 * pieces[0] + 2 * pieces[1] + pieces[0] * pieces[1] * pieces[2];
    }
    return sum;
}
