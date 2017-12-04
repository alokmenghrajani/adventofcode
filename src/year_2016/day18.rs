// There's probably a way to get to the count without having to build the string for each row.
// The pattern is a sierpinski triangle. I however took the naive approach.
pub fn solve(input: &str) {
    assert_eq!(expand("..^^.".to_string(), 1), ".^^^^".to_string());
    assert_eq!(expand("..^^.".to_string(), 2), "^^..^".to_string());

    assert_eq!(_solve(".^^.^.^^^^", 10), 38);

    println!("part 1: {}", _solve(input.trim(), 40));
    println!("part 2: {}", _solve(input.trim(), 400000));
}

fn _solve(input: &str, n: usize) -> usize {
    let mut i = input.to_string();
    let mut r = i.bytes().filter(|x| *x == b'.').count();
    for _ in 1..n {
        i = expand(i, 1);
        r += i.bytes().filter(|x| *x == b'.').count();
    }
    r
}

// Computes the next row given the current row.
// note: it would be more efficient to alternate between buffers instead of allocating a new
// buffer for each row. We also convert to/from utf8 too often.
fn expand(input: String, n: usize) -> String {
    if n == 0 {
        return input;
    }
    let b: Vec<u8> = input.bytes().collect();
    let len = b.len();
    let mut r = Vec::with_capacity(len);
    for i in 0..len {
        // the 4 rules in the instructions boil down to left xor right.
        let left = *b.get((i as isize - 1) as usize).unwrap_or(&b'.');
        let right = *b.get(i + 1).unwrap_or(&b'.');
        r.push(if left == right { b'.' } else { b'^' });
    }
    expand(String::from_utf8(r).unwrap(), n - 1)
}
