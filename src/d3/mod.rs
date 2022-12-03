use itertools::Itertools;
use std::{fs::File, path::Path, io::{BufReader, BufRead}, cmp::min};


const UPPER_START: u8 = 65;
const LOWER_START: u8 = 97;

/// Returns the priority of the character.
/// 
/// * `alpha` - an ASCII alphabetic character. (A-Z, a-z)
fn priority(alpha: char) -> u8 {
    let number = alpha as u8;
    if alpha.is_uppercase() {
        number - UPPER_START + 27
    }
    else {
        number - LOWER_START + 1
    }
}

/// Part 1: returns the priority sum of item that appear in both compartments of a rucksack.
fn sum_items_in_both(file: File) -> u32 {
    let reader = BufReader::new(file);
    let input = reader.lines().map_while(|res| res.ok());

    let mut total = 0;
    for rucksack in input {
        let (comp_1, comp_2) = rucksack.split_at(rucksack.len()/2);
        let mut in_both: Vec<char> = Vec::with_capacity(min(comp_1.len(), comp_2.len()));
        for c1 in comp_1.chars() {
            let mut sum: u32 = 0;
            'search: for c2 in comp_2.chars() {
                if c1 == c2 {
                    if !in_both.contains(&c1) {
                        sum += priority(c1) as u32;
                        in_both.push(c1);
                        break 'search;
                    } 
                }
            }
            total += sum;
        }  
    }
    total
}

/// Part 2: return the priority sum of all the group badges.
fn sum_group_badges(file: File) -> u32 {
    let reader = BufReader::new(file);
    let input = reader.lines().map_while(|res| res.ok());

    let mut total = 0;
    for (elf1, elf2, elf3) in input.tuples() {
        'search: for c1 in elf1.chars() {
            if elf2.contains(c1) && elf3.contains(c1) {
                total += priority(c1) as u32;
                break 'search;
            }
        }
    }
    total
}

/// Runs the day 3 program.
pub fn main(path: &String) {
    let path = Path::new(path.as_str());
    match File::open(path) {
        Ok(file) => {
            let part_1 = sum_items_in_both(file);
            println!("part 1: {}", part_1);
        },
        Err(err) => eprintln!("error: Could not read file: {}", err),
    };
    match File::open(path) {
        Ok(file) => {
            let part_1 = sum_group_badges(file);  
            println!("part 2: {}", part_1);
        },
        Err(err) => eprintln!("error: Could not read file: {}", err),
    };
}