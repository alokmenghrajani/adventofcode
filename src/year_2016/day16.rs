// This puzzle wasn't very tricky, you just have to be careful about not re-allocating/copying
// memory around for no reason.
//
// My solution expands the input string, truncates it and then recursively computes the checksum.

pub fn solve(input: &str) {
    let mut t = str_to_vec("1");
    expand(&mut t);
    assert_eq!(t, str_to_vec("100"));

    t = str_to_vec("0");
    expand(&mut t);
    assert_eq!(t, str_to_vec("001"));

    t = str_to_vec("11111");
    expand(&mut t);
    assert_eq!(t, str_to_vec("11111000000"));

    t = str_to_vec("111100001010");
    expand(&mut t);
    assert_eq!(t, str_to_vec("1111000010100101011110000"));

    t = str_to_vec("110010110100");
    checksum(&mut t);
    assert_eq!(vec_to_str(&t), "100");
    assert_eq!(_solve("10000", 20), "01100");

    println!("part 1: {:?}", _solve(input, 272));
    println!("part 2: {:?}", _solve(input, 35651584));
}

fn _solve(input: &str, len: usize) -> String {
    let mut t = str_to_vec(input);
    t.reserve(len * 2);
    while t.len() < len {
        expand(&mut t);
    }
    t.truncate(len);
    checksum(&mut t);
    vec_to_str(&t)
}

// Helper function to convert a string of "0" and "1" into a Vec<bool>.
fn str_to_vec(s: &str) -> Vec<bool> {
    s.to_string().into_bytes().iter().map(|x| *x == b'1').collect()
}

// Helper function to go the other way.
fn vec_to_str(v: &Vec<bool>) -> String {
    String::from_utf8(v.iter().map(|x| if *x { b'1' } else { b'0' }).collect()).unwrap()
}

// Expand the vector in-place.
fn expand(t: &mut Vec<bool>) {
    let l = t.len();
    t.push(false);
    for i in (0..l).rev() {
        let v = !t[i];
        t.push(v);
    }
}

fn checksum(t: &mut Vec<bool>) {
    let l = t.len();
    if l % 2 == 1 {
        return;
    }
    //    let mut r = Vec::with_capacity(l / 2);
    let mut i = 0;
    let mut j = 0;
    while i < l {
        t[j] = t[i] == t[i + 1];
        i += 2;
        j += 1;
    }
    t.truncate(j);
    checksum(t)
}
