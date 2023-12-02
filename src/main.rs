use clap::Parser;
use cli::Cli;

mod cli;
mod common;

mod aoc;

fn main() {
    let cli = Cli::parse();

    aoc::run(cli);
}
