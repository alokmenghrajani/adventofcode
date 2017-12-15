pub fn run(gen_a: u64, gen_b: u64) {
    let part1 = solve(gen_a, 1, gen_b, 1, 40_000_000);
    println!("part 1: {}", part1);
    assert_eq!(part1, 619);

    let part2 = solve(gen_a, 4, gen_b, 8, 5_000_000);
    println!("part 2: {}", part2);
    assert_eq!(part2, 290);
}

fn solve(gen_a: u64, multiples_a: u64, gen_b: u64, multiples_b: u64, iter: usize) -> usize {
    let gen_a = Gen::new(gen_a, multiples_a, 16807, 2147483647);
    let gen_b = Gen::new(gen_b, multiples_b, 48271, 2147483647);

    let mut count = 0;
    for (v1, v2) in gen_a.zip(gen_b).take(iter) {
        if (v1 & 0xffff) == (v2 & 0xffff) {
            count += 1;
        }
    }
    return count;
}

struct Gen {
    value: u64,
    multiples: u64,
    factor: u64,
    remainder: u64,
}

impl Gen {
    fn new(initial_value: u64, multiples: u64, factor: u64, remainder: u64) -> Gen {
        return Gen {
            value: initial_value,
            multiples: multiples,
            factor: factor,
            remainder: remainder,
        };
    }
}

impl Iterator for Gen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * self.factor) % self.remainder;
            if self.value % self.multiples == 0 {
                return Some(self.value);
            }
        }
    }
}
