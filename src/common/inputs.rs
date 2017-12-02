use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Loads the file at inputs/<year>/day<number>.txt.
// Keep in mind that the input files are unique to each user.
pub fn read(year: u32, day: u8) -> Vec<String> {
    // todo: we could stat the file and pre-allocate a String with the right capacity.
    let mut r = Vec::with_capacity(1000);
    let path = format!("./inputs/{}/day{:02}.txt", year, day);
    let fp = File::open(&path).expect(&format!("Can't open {}", path));
    let fp = BufReader::new(fp);
    for line in fp.lines() {
        r.push(line.unwrap());
    }
    return r;
}

// Loads the file at inputs/<year>/day<number>.txt.
// Keep in mind that the input files are unique to each user.
pub fn read_first_line(year: u32, day: u8) -> Vec<u8> {
    // todo: we could stat the file and pre-allocate a String with the right capacity.
    let mut r = Vec::with_capacity(10_000);
    let path = format!("./inputs/{}/day{:02}.txt", year, day);
    let fp = File::open(&path).expect(&format!("Can't open {}", path));
    r.extend_from_slice(&fp.bytes()
        .map(|c| c.unwrap())
        .take_while(|c| *c != b'\n')
        .collect::<Vec<u8>>());
    return r;
}