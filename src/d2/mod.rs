use std::{path::Path, fs::File, io::{BufReader, BufRead}};

/// The hand shape module.
pub mod game;
use game::{Shape, GameResult};


/// Calculates the scores from `input` and returns the results.
///
///  * `input` - An input buffer
fn calculate_scores<Input>(input: Input) -> (u32, u32) where Input: Iterator<Item = String> {
    let mut score: (u32, u32) = (0, 0);
    for line in input {
        let split: Vec<&str> = line.split(' ').collect();
        if split.len() == 2 {
            score.0 += score_part_1(split[0], split[1]);
            score.1 += score_part_2(split[0], split[1]);
        }
    }
    score
}

/// Calculates the score from part 1 and returns the result.
///
/// * `opponent_move` - A string representing the opponents move.
/// * `your_move` - A string representing your move.
fn score_part_1(opponent_move: &str, your_move: &str) -> u32 {
    let opp_shape = Shape::try_from(opponent_move).unwrap();
    let your_shape = Shape::try_from(your_move).unwrap();
    let result = your_shape.result(&opp_shape);
    return your_shape as u32 + result as u32;
}

/// Calculates the score from part 2 and returns the result.
///
/// * `opponent_move` - A string representing the opponents move.
/// * `desired_result` - A string representing the desired result of the game.
fn score_part_2(opponent_move: &str, desired_result: &str) -> u32 {
    let opp_shape = Shape::try_from(opponent_move).unwrap();
    let result = GameResult::try_from(desired_result).unwrap();
    let your_shape = opp_shape.solution(&result);
    return your_shape as u32 + result as u32;
}

/// Runs the Day 2 program.
pub fn main(path: &String) {
    let path = Path::new(path.as_str());
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let input = reader.lines()
                .map_while(|res| res.ok());

            let (part_1, part_2) = calculate_scores(input);
            println!("part 1: {}", part_1);
            println!("part 2: {}", part_2);
        },
        Err(err) => eprintln!("error: Could not read file: {}", err),
    };
}