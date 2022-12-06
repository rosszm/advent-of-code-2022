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
    /// Day 2: Rock Paper Scissors
    D2 {
        /// The path to the input file
        path: String
    },
    /// Day 3: Rucksack Reorganization
    D3 {
        /// The path to the input file
        path: String
    },
    /// Day 4: Camp Cleanup
    D4 {
        /// The path to the input file
        path: String
    },
    /// Day 5: Supply Stacks
    D5 {
        /// The path to the input file
        path: String
    },
    /// Day 6: Tuning Trouble
    D6 {
        /// The path to the input file
        path: String
    },
}
impl Days {
    fn run(&self) {
        match self {
            Days::D1 { path } => aoc2022::d1::main(path),
            Days::D2 { path } => aoc2022::d2::main(path),
            Days::D3 { path } => aoc2022::d3::main(path),
            Days::D4 { path } => aoc2022::d4::main(path),
            Days::D5 { path } => aoc2022::d5::main(path),
            Days::D6 { path } => aoc2022::d6::main(path),
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
