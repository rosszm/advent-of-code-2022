use std::cmp::Ordering;


/// The result of a game of Rock, Paper, Scissors.
#[derive(PartialEq, Eq, Debug)]
pub enum GameResult {
    /// A loss result; worth 0 points.
    Loss = 0,
    /// A draw result; worth 3 points.
    Draw = 3,
    /// A win result; worth 6 points.
    Win = 6,
}
impl From<Ordering> for GameResult {
    fn from(ord: Ordering) -> Self {
        match ord {
            Ordering::Less => GameResult::Loss,
            Ordering::Equal => GameResult::Draw,
            Ordering::Greater => GameResult::Win,
        }
    }
}
impl TryFrom<&str> for GameResult {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Loss),
            "Y"  => Ok(Self::Draw),
            "Z"  => Ok(Self::Win),
            _ => Err("Could not parse action: char is not valid".to_string())
        }
    }
}