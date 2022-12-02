use std::{path::Path, io::{self, BufRead}, fs::File};

/// Parses an input file and returns a collection, where each value is the total number of calories
/// carried by an elf.
///
/// * `path` - A path to a file of food item calories, where an empty line signals food carried by a
///     different elf.
fn read_list_from_file<P>(path: P) -> io::Result<Vec<u32>> where P: AsRef<Path> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut list = Vec::with_capacity(reader.capacity());
    let mut is_new = true;
    for line in reader.lines().map_while(|res| res.ok()) {
        if !line.is_empty() {
            if let Ok(value) = line.parse::<u32>() {
                if is_new {
                    list.push(value)
                }
                else {
                    let cur = list.len() - 1;
                    list[cur] += value
                }
            }
            is_new = false;
        }
        else {
            is_new = true;
        }
    }
    Ok(list)
}

/// Returns the maximum number of calories carried by any elf.
fn max_calorie_count<'vec>(calories: &'vec Vec<u32>) -> &'vec u32 {
    return calories.iter().max().unwrap();
}

fn max_calorie_count_of(mut calories: Vec<u32>, num: usize) -> u32 {
    let mut result: Vec<u32> = Vec::new();
    calories.sort();
    for _ in 0..num {
        if let Some(value) = calories.pop() {
            result.push(value)
        }
    }
    result.iter().sum()
}

/// Runs the Day 1 program.
pub fn main(path: &String) {
    let path = Path::new(path.as_str());
    match read_list_from_file(path) {
        Ok(list) => {
            let part_1 = max_calorie_count(&list);
            println!("part 1: {}", part_1);

            let part_2 = max_calorie_count_of(list.clone(), 3);
            println!("part 2: {}", part_2);
        },
        Err(err) => eprintln!("error: Could not read file: {}", err)
    };
}