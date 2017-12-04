use std::env;

mod common;
mod year_2015;
mod year_2017;

extern crate crypto;
extern crate regex;

use common::inputs;

fn main() {
    // Convert argv to a Vec<String>, so we can access elements using [] notation.
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 3 {
        panic!("Usage: cargo run <year> <day>");
    }

    // In general, Rust can infer types for local variables. Here, we need to tell the type
    // inference which type we want to parse the String into because a String can be parsed into
    // many different types.
    let year: u32 = argv[1].parse().expect("Year must be a number");
    let day: u8 = argv[2].parse().expect("Day must be a number");
    println!("Running year {}, day {}", year, day);

    match (year, day) {
        // 2015
        (2015, 1) => year_2015::day01::run(&inputs::read_first_line(year, day)),
        (2015, 2) => year_2015::day02::run(inputs::read(year, day)),
        (2015, 3) => year_2015::day03::run(&inputs::read_first_line(year, day)),
        (2015, 4) => year_2015::day04::run("bgvyzdsv"),
        (2015, _) => println!("work in progress..."),

        // 2016
        (2016, _) => {
            println!("TODO: move code over from \
                      https://github.\
                      com/alokmenghrajani/random_stuff/tree/master/adventofcode/2016")
        }

        // 2017
        (2017, 1) => year_2017::day01::run(&inputs::read_first_line(year, day)),
        (2017, 2) => year_2017::day02::run(inputs::read(year, day)),
        (2017, 3) => year_2017::day03::run(368078),

        (_, _) => panic!("Not implemented"),
    }
}
