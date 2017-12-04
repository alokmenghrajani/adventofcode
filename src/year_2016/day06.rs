use std::collections::HashMap;

pub fn solve(input: &str) {
    let test_input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
    assert_eq!(part1(&test_input, 6), "easter");
    println!("part 1: {}", part1(input, 8));

    assert_eq!(part2(&test_input, 6), "advent");
    println!("part 2: {}", part2(input, 8));

}

fn part1(input: &str, l: usize) -> String {
    _solve(&input.trim().split("\n").map(|x| x.as_bytes()).collect(), l).0
}

fn part2(input: &str, l: usize) -> String {
    _solve(&input.trim().split("\n").map(|x| x.as_bytes()).collect(), l).1
}

fn _solve(inputs: &Vec<&[u8]>, l: usize) -> (String, String) {
    // initialize an array of maps. The maps will track the frequency of each character.
    let mut counts = vec![];
    for _ in 0..l {
        counts.push(HashMap::new());
    }
    // fill the frequency counts.
    for input in inputs {
        for i in 0..l {
            *counts[i].entry(input[i]).or_insert(0) += 1;
        }
    }
    // find the most and least common character in each map.
    let mut part1 = String::new();
    let mut part2 = String::new();
    for i in 0..l {
        let elements: Vec<(&u8, &i32)> = counts[i].iter().collect();
        let mut min = elements[0];
        let mut max = elements[0];
        for e in elements {
            if e.1 > max.1 {
                max = e;
            }
            if e.1 < min.1 {
                min = e;
            }
        }
        part1.push(*max.0 as char);
        part2.push(*min.0 as char);
    }
    (part1, part2)
}
