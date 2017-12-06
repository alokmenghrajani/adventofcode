use std::env;

mod common;
mod year_2015;
mod year_2016;
mod year_2017;

extern crate crypto;
extern crate fancy_regex;
extern crate regex;

#[macro_use]
extern crate lazy_static;

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
        (2015, 5) => year_2015::day05::run(inputs::read(year, day)),
        (2015, 6) => year_2015::day06::run(inputs::read(year, day)),
        (2015, 7) => year_2015::day07::run(inputs::read(year, day)),
        (2015, 8) => year_2015::day08::run(inputs::read(year, day)),
        (2015, 9) => year_2015::day09::run(inputs::read(year, day)),
        (2015, _) => println!("work in progress..."),

        // 2016
        (2016, 1) => year_2016::day01::run(&inputs::grab_local_input(year, day)),
        (2016, 2) => year_2016::day02::run(&inputs::grab_local_input(year, day)),
        (2016, 3) => year_2016::day03::run(&inputs::grab_local_input(year, day)),
        (2016, 4) => year_2016::day04::run(&inputs::grab_local_input(year, day)),
        (2016, 5) => year_2016::day05::run("ojvtpuvg"),
        (2016, 6) => year_2016::day06::solve(&inputs::grab_local_input(year, day)),
        (2016, 7) => year_2016::day07::solve(&inputs::grab_local_input(year, day)),
        (2016, 8) => year_2016::day08::solve(&inputs::grab_local_input(year, day)),
        (2016, 9) => year_2016::day09::solve(&inputs::grab_local_input(year, day)),
        (2016, 10) => year_2016::day10::solve(&inputs::grab_local_input(year, day)),
        //(2016, 12) => year_2016::day12::solve(&inputs::grab_local_input(year, day)),
        (2016, 14) => year_2016::day14::solve("qzyelonm"),
        (2016, 15) => year_2016::day15::solve(&inputs::grab_local_input(year, day)),
        (2016, 16) => year_2016::day16::solve("01111001100111011"),
        (2016, 18) => year_2016::day18::solve(&inputs::grab_local_input(year, day)),
        (2016, 19) => year_2016::day19::solve(3005290),
        (2016, 20) => year_2016::day20::solve(&inputs::grab_local_input(year, day)),
        (2016, 21) => year_2016::day21::solve(&inputs::grab_local_input(year, day), "abcdefgh", "fbgdceah"),
        //(2016, 23) => year_2016::day23::solve(&inputs::grab_local_input(year, day)),

        // 2017
        (2017, 1) => year_2017::day01::run(&inputs::read_first_line(year, day)),
        (2017, 2) => year_2017::day02::run(inputs::read(year, day)),
        (2017, 3) => year_2017::day03::run(368078),
        (2017, 4) => year_2017::day04::run(inputs::read(year, day)),
        (2017, 5) => year_2017::day05::run(inputs::read(year, day)),
        (2017, 6) => year_2017::day06::run(&inputs::read_first_line(year, day)),

        (_, _) => panic!("Not implemented :("),
    }
}
