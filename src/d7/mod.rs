use std::{io::{BufReader, BufRead}, fs::File};


const MAX_SIZE: usize = 100000;
const TOTAL_DISK: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;


/// Returns the sum of directory sizes that are less than or equal to a given size.
fn sum_dirs_size_le(dirs: &Vec<usize>, size: usize) -> usize {
    dirs.iter()
        .filter(|&&s| s <= size)
        .sum::<usize>()
}

/// Returns the size of the smallest directory required to be deleted based the total disk size and
/// the size of an update.
fn dir_to_delete_size(dirs: &Vec<usize>, disk_size: usize, update_size: usize) -> usize {
    let free_space = disk_size as i128 - dirs[0] as i128;
    let required_space = update_size as i128 - free_space;
    let mut dir_size: usize = 0;
    if required_space > 0 {
        dir_size += dirs.iter()
            .filter(|&&s| s >= required_space as usize)
            .min()
            .unwrap()
    }
    dir_size
}

/// Runs the day 7 program.
pub fn main(path: &String) {
    match File::open(path) {
        Ok(file) => {
            let mut sizes: Vec<usize> = Vec::new();
            let mut nav_stack: Vec<usize> = Vec::new();

            let reader = BufReader::new(file);
            for line in reader.lines().map_while(|res| res.ok()) {
                let argv: Vec<&str> = line.split(' ').collect();
                if argv[0] == "$" {
                    if argv[1] == "cd" {
                        if argv[2] == ".." {
                            let prev = nav_stack.pop().unwrap();
                            let cwd = *nav_stack.last().unwrap();
                            sizes[cwd] += sizes[prev];
                        }
                        else {
                            sizes.push(0);
                            nav_stack.push(sizes.len() - 1);
                        }
                    }
                } else if let Ok(file_size) = argv[0].parse::<usize>() {
                    let cwd = *nav_stack.last().unwrap();
                    sizes[cwd] += file_size;
                }
            }
            while nav_stack.len() > 1 {
                let prev = nav_stack.pop().unwrap();
                let cwd = *nav_stack.last().unwrap();
                sizes[cwd] += sizes[prev];
            }

            println!("part 1: {}", sum_dirs_size_le(&sizes, MAX_SIZE));
            println!("part 2: {}", dir_to_delete_size(&sizes, TOTAL_DISK, UPDATE_SIZE));
        }
        Err(err) => println!("error: Could not read file: {}", err),
    }
}