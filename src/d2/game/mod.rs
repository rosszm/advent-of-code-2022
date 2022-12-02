use std::cmp::Ordering;

/// The result module.
mod result;
pub use result::GameResult;


/// Represents a hand shape in a game of Rock, Paper, Scissors.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Shape {
    /// The rock shape; worth 1 point.
    Rock = 1,
    /// The paper shape; worth 2 points.
    Paper = 2,
    /// The scissors shape; worth 3 points.
    Scissors = 3,
}
impl Shape {
   /// Returns the result of a game between this shape and another shape.
    pub fn result(&self, other: &Self) -> GameResult {
        GameResult::from(self.cmp(other))
    }

    /// Returns a shape that that will produce the desired game result against this shape.
    ///
    /// # Example
    /// ```
    /// use aoc2022::d2::game::{Shape, GameResult};
    ///
    /// assert_eq!(Shape::Rock.solution(&GameResult::Win), Shape::Paper);
    /// ```
    pub fn solution(&self, result: &GameResult) -> Self {
        let mut solution: Option<Shape> = None;
        for shape in Shape::iter() {
            if &shape.result(self) == result {
                solution = Some(shape)
            }
        }
        solution.unwrap()
    }

    /// Returns a vector of all shapes.
    fn iter() -> Iterator { Iterator::new() }
}
impl TryFrom<&str> for Shape {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y"  => Ok(Self::Paper),
            "C" | "Z"  => Ok(Self::Scissors),
            _ => Err("Could not parse action: char is not valid".to_string())
        }
    }
}
impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Ordering::Equal,
                Shape::Paper => Ordering::Less,
                Shape::Scissors => Ordering::Greater,
            },
            Shape::Paper => match other {
                Shape::Rock => Ordering::Greater,
                Shape::Paper => Ordering::Equal,
                Shape::Scissors => Ordering::Less,
            },
            Shape::Scissors => match other {
                Shape::Rock => Ordering::Less,
                Shape::Paper => Ordering::Greater,
                Shape::Scissors => Ordering::Equal,
            },
        }
    }
}

/// An iterator for the shape enum.
///
/// This iterator
struct Iterator {
    cur: Option<Shape>,
}
impl Iterator {
    fn new() -> Self {
        Iterator { cur: Some(Shape::Rock) }
    }
}
impl std::iter::Iterator for Iterator {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.cur;
        self.cur = if let Some(shape) = result {
            match shape {
                Shape::Rock => Some(Shape::Paper),
                Shape::Paper => Some(Shape::Scissors),
                Shape::Scissors => None,
            }
        } else { None };
        return result;
    }
}