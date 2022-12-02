use clap::{Parser, command, Subcommand};

/// The Arguments for the Advent of Code 2022 program.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Command {
    /// The day argument; represents the day of the program to run.
    #[command(subcommand)]
    day: Days,
}

/// Sub-commands for each day in Advent of Code 2022.
#[derive(Debug, Subcommand)]
enum Days {
    /// Day 1: Calorie Counting
    D1 {
        /// The path to the input file
        path: String
    },
}
impl Days {
    fn run(&self) {
        match self {
            Days::D1 { path } => aoc2022::d1::main(path),
        }
    }
}

/// # Advent of Code 2022
///
/// The main program for Advent of Code 2022. This program acts as a single entrypoint
/// for all 25 days.
fn main() {
    let cmd = Command::parse();
    cmd.day.run();
}