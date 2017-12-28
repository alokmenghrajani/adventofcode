use regex::Regex;
use std::i64;

pub fn run(input: Vec<String>) {
    let part1 = solve(input.clone(), 100, None);
    println!("part 1: {}", part1);
    assert_eq!(part1, 222870);

    let part2 = solve(input.clone(), 100, Some(500));
    println!("part 2: {}", part2);
    assert_eq!(part2, 117936);
}

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn solve(input: Vec<String>, teaspoons: u64, calories: Option<i64>) -> i64 {
    // Parse the input.
    let mut ingredients: Vec<Ingredient> = Vec::new();

    let re = Regex::new(r"^.+?: capacity (-?\d+?), durability (-?\d+?), flavor (-?\d+?), texture (-?\d+?), calories (-?\d+?)$").unwrap();
    for line in input.iter() {
        match re.captures(line) {
            Some(cap) => {
                let capacity: i64 = cap.get(1).unwrap().as_str().parse().unwrap();
                let durability: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
                let flavor: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
                let texture: i64 = cap.get(4).unwrap().as_str().parse().unwrap();
                let calories: i64 = cap.get(5).unwrap().as_str().parse().unwrap();
                ingredients.push(Ingredient {
                    capacity: capacity,
                    durability: durability,
                    flavor: flavor,
                    texture: texture,
                    calories: calories,
                });
            }
            None => {
                panic!("invalid input: {}", line);
            }
        }
    }

    // Simplex algorithm would probably work here and give a result much faster.
    let mut best_value = i64::MIN;
    let mut current = Vec::new();
    for _ in 0..ingredients.len() {
        current.push(0);
    }
    solve_part1_recursive(&ingredients,
                          0,
                          teaspoons,
                          calories,
                          &mut current,
                          &mut best_value);

    return best_value;
}

fn solve_part1_recursive(ingredients: &Vec<Ingredient>,
                         offset: usize,
                         teaspoons: u64,
                         desired_calories: Option<i64>,
                         current: &mut Vec<u64>,
                         best_value: &mut i64) {
    assert!(offset < ingredients.len());
    if offset == (ingredients.len() - 1) {
        // we take teaspoons of the remaining ingredient
        current[offset] = teaspoons;

        // we are done, check if we beat best_value
        let mut capacity: i64 = ingredients.iter()
            .enumerate()
            .map(|(i, ingredient)| (current[i] as i64) * ingredient.capacity)
            .sum();
        if capacity < 0 {
            capacity = 0;
        }

        let mut durability: i64 = ingredients.iter()
            .enumerate()
            .map(|(i, ingredient)| (current[i] as i64) * ingredient.durability)
            .sum();
        if durability < 0 {
            durability = 0;
        }

        let mut flavor: i64 = ingredients.iter()
            .enumerate()
            .map(|(i, ingredient)| (current[i] as i64) * ingredient.flavor)
            .sum();
        if flavor < 0 {
            flavor = 0;
        }

        let mut texture: i64 = ingredients.iter()
            .enumerate()
            .map(|(i, ingredient)| (current[i] as i64) * ingredient.texture)
            .sum();
        if texture < 0 {
            texture = 0;
        }



        let t = capacity * durability * flavor * texture;
        if t > *best_value {
            match desired_calories {
                Some(v) => {
                    let calories: i64 = ingredients.iter()
                        .enumerate()
                        .map(|(i, ingredient)| (current[i] as i64) * ingredient.calories)
                        .sum();
                    if calories == v {
                        *best_value = t;
                    }
                }
                None => {
                    *best_value = t;
                }
            }
        }
        return;
    }

    // take between 0 and teaspoons
    for i in 0..(teaspoons + 1) {
        current[offset] = i;
        solve_part1_recursive(ingredients,
                              offset + 1,
                              teaspoons - i,
                              desired_calories,
                              current,
                              best_value);
    }
}
