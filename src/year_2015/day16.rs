use std::collections::HashMap;
use std::u64;

pub fn run(input: Vec<String>) {
    let mut needle = HashMap::new();
    needle.insert("children".to_string(), (3, 3));
    needle.insert("cats".to_string(), (7, 7));
    needle.insert("samoyeds".to_string(), (2, 2));
    needle.insert("pomeranians".to_string(), (3, 3));
    needle.insert("akitas".to_string(), (0, 0));
    needle.insert("vizslas".to_string(), (0, 0));
    needle.insert("goldfish".to_string(), (5, 5));
    needle.insert("trees".to_string(), (3, 3));
    needle.insert("cars".to_string(), (2, 2));
    needle.insert("perfumes".to_string(), (1, 1));

    let part1 = solve(input.clone(), needle.clone());
    println!("part 1: {}", part1);
    assert_eq!(part1, 373);

    needle.insert("cats".to_string(), (8, u64::MAX));
    needle.insert("pomeranians".to_string(), (0, 2));
    needle.insert("goldfish".to_string(), (0, 4));
    needle.insert("trees".to_string(), (4, u64::MAX));

    let part2 = solve(input.clone(), needle.clone());
    println!("part 2: {}", part2);
    assert_eq!(part2, 260);
}

fn solve(input: Vec<String>, needle: HashMap<String, (u64, u64)>) -> usize {
    let mut aunts: Vec<HashMap<String, u64>> = Vec::new();

    // Parse the input.
    for line in input.iter() {
        let line: String = line.split(": ").skip(1).collect::<Vec<&str>>().join(": ");
        let compounds: Vec<&str> = line.split(", ").collect();
        let mut m = HashMap::new();
        for compound in compounds {
            let mut t = compound.split(": ");
            let name = t.next().unwrap().to_string();
            let value: u64 = t.next().unwrap().parse().unwrap();
            m.insert(name, value);
        }
        aunts.push(m);
    }

    // search aunts for a match with needle
    'outer: for (n, aunt) in aunts.iter().enumerate() {
        for (compound, &value) in needle.iter() {
            match aunt.get(compound) {
                None => {}
                Some(&v) => {
                    if v < value.0 || v > value.1 {
                        continue 'outer;
                    }
                }
            }
        }
        return n + 1;
    }

    return 0;
}
