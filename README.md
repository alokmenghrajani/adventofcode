# AdventOfCode
My (commented) solutions to [AoC](https://adventofcode.com/) 2015-2017 in [Rust](https://www.rust-lang.org/).

           o
          /|\
         //|\\
        ///|\\\
       ////|\\\\
      /////|\\\\\
      0 0 ||| 0 0
        __|||__


## Running
`cargo run <year> <day>`

## Noteworthy
For `assembunny` (days 12 & 23 in 2016), I implemented a JIT VM. It sounds crazy (and totally overkill), but it was a lot of fun :)

## Other random remarks
- Writing the input parsing code takes a whole bunch of time. I should do something about that.
- I should look for duplicated logic and refactor code to leverage some utility code. E.g. wrap-around slice/vector might be useful for a bunch of puzzles.
- I still feel I'm sometimes fighting the borrow checker. I need to work on my Rust skills, and perhaps
do some things differently.
