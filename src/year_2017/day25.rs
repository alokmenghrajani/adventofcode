use std::collections::HashMap;

pub fn run() {
    // For the last day, it was easier to directly convert the input into cod else
    // multi-selection to the rescue!
    let mut pos = 0;
    let mut state = 'A';
    let mut tape: HashMap<i64, i64> = HashMap::new();

    for _ in 0..12629077 {
        let t = tape.entry(pos).or_insert(0);
        if state == 'A' {
            if *t == 0 {
                *t = 1;
                pos += 1;
                state = 'B';
            } else {
                *t = 0;
                pos -= 1;
                state = 'B';
            }
        } else if state == 'B' {
            if *t == 0 {
                *t = 0;
                pos += 1;
                state = 'C';
            } else {
                *t = 1;
                pos -= 1;
                state = 'B';
            }
        } else if state == 'C' {
            if *t == 0 {
                *t = 1;
                pos += 1;
                state = 'D';
            } else {
                *t = 0;
                pos -= 1;
                state = 'A';
            }
        } else if state == 'D' {
            if *t == 0 {
                *t = 1;
                pos -= 1;
                state = 'E';
            } else {
                *t = 1;
                pos -= 1;
                state = 'F';
            }
        } else if state == 'E' {
            if *t == 0 {
                *t = 1;
                pos -= 1;
                state = 'A';
            } else {
                *t = 0;
                pos -= 1;
                state = 'D';
            }
        } else if state == 'F' {
            if *t == 0 {
                *t = 1;
                pos += 1;
                state = 'A';
            } else {
                *t = 1;
                pos -= 1;
                state = 'E';
            }
        }
    }

    let sum: i64 = tape.values().sum();
    println!("part 1: {}", sum);
    assert_eq!(sum, 3745);
}
