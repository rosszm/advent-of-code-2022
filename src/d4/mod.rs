use std::{fs::File, io::{self, BufReader, BufRead}, num::ParseIntError, ops::RangeInclusive};


/// The section range structure.
///
/// This structure represents an inclusive range
struct SectionRange {
    start: u8,
    end: u8,
}
impl SectionRange {
    /// Creates a new section range from a start and end integer (inclusive).
    fn new(start: u8, end: u8) -> Self {
        SectionRange {start, end}
    }

    fn as_range_inclusive(&self) -> RangeInclusive<u8> {
        self.start..=self.end
    }

    /// Returns whether or not this section range full contains another section
    /// range.
    fn full_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Returns whether or not this section range overlaps with another section
    /// range.
    fn overlaps(&self, other: &Self) -> bool {
        let self_range = self.as_range_inclusive();
        let other_range = other.as_range_inclusive();

        other_range.contains(&self.start)
            ||  other_range.contains(&self.end)
            || self_range.contains(&other.start)
            || self_range.contains(&other.end)
    }
}
impl TryFrom<&str> for SectionRange {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split('-').collect();
        let start = split[0].parse::<u8>()?;
        let end = split[1].parse::<u8>()?;
        Ok(SectionRange::new(start, end))
    }
}

/// Returns the number of pairs where the assigned sections of one elf fully contains the
/// other.
///
/// * `path` the path of an input file.
fn fully_contained_count(path: &String) -> io::Result<(u16, u16)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().map_while(|res| res.ok());

    let mut fully_contains_count = 0;
    let mut overlaps_count = 0;

    for line in input {
        let split: Vec<&str> = line.split(',').collect();
        let elf1 = SectionRange::try_from(split[0]).unwrap();
        let elf2 = SectionRange::try_from(split[1]).unwrap();

        if elf1.full_contains(&elf2) || elf2.full_contains(&elf1) {
            fully_contains_count += 1;
        }
        if elf1.overlaps(&elf2) || elf2.overlaps(&elf1) {
            overlaps_count += 1;
        }
    }
    Ok((fully_contains_count, overlaps_count))
}

/// Runs the day 4 program.
pub fn main(path: &String) {
    match fully_contained_count(path) {
        Ok((part1, part2)) => {
            println!("part 1: {}", part1);
            println!("part 2: {}", part2);
        },
        Err(err) => println!("error: Could not read file: {}", err),
    }
}