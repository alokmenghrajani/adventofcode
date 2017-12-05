use std::iter::FromIterator;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 386);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 208);
}

fn solve_part1(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    for line in input.iter() {
        // split each line on space
        let pieces: Vec<&str> = line.split(" ").collect();

        // Search for duplicates. An alternative to using two loops is to stuff each piece in a
        // set and then check if the set's size is the same as the number of pieces.
        let mut ok = true;
        'outer: for i in 0..pieces.len() {
            for j in (i + 1)..pieces.len() {
                if pieces[i] == pieces[j] {
                    ok = false;
                    break 'outer;
                }
            }
        }
        if ok {
            sum += 1;
        }
    }
    return sum;
}

fn solve_part2(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    for line in input.iter() {
        // split each line on space
        let pieces: Vec<&str> = line.split(" ").collect();

        // We can find anagrams by sorting each word.
        let pieces: Vec<String> = pieces.iter().map(|s| normalize(s)).collect();

        // Search for duplicates. An alternative to using two loops is to stuff each piece in a
        // set and then check if the set's size is the same as the number of pieces.
        let mut ok = true;
        'outer: for i in 0..pieces.len() {
            for j in (i + 1)..pieces.len() {
                if pieces[i] == pieces[j] {
                    ok = false;
                    break 'outer;
                }
            }
        }
        if ok {
            sum += 1;
        }
    }
    return sum;
}

fn normalize(anagram: &str) -> String {
    let mut chars: Vec<char> = anagram.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    return String::from_iter(chars);
}
