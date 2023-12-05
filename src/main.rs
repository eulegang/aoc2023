use clap::Parser;
use cli::Cli;

macro_rules! day {
    ($day: ident) => {
        pub fn run() {
            let input = $day::parse(include_bytes!("input.txt"));
            let output = $day::calc(input);

            println!("{output}");
        }
    };

    ($day: ident, $meth: ident) => {
        pub fn run() {
            let input = $day::parse(include_bytes!("input.txt"));
            let output = $day::$meth(input);

            println!("{output}");
        }
    };
}

mod cli;
mod common;

mod aoc;

fn main() {
    //let cli = Cli::parse();

    aoc::run();
}
