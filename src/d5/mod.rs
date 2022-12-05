use std::{fs::File, io::{BufReader, BufRead}};

/// A trait that specifies how items should be rearranged.
trait Rearrange {
    /// Moves a number of items from a source to a destination, one-by-one.
    ///
    /// * `quantity` - the number of items to move
    /// * `src` - the source index (where to get the items)
    /// * `dst` - the destination index (where to put the items)
    fn rearrange(&mut self, step: &Move);

    /// Moves a number of items from a source to a destination. This function moves the items as a
    /// group rather than one-by-one.
    ///
    /// * `quantity` - the number of items to move
    /// * `src` - the source index (where to get the items)
    /// * `dst` - the destination index (where to put the items)
    fn rearrange_multi(&mut self, step: &Move);
}

impl Rearrange for Vec<Vec<char>> {
    fn rearrange(&mut self, step: &Move) {
        for _ in 0..step.quantity {
            if let Some(item) = self[step.src].pop() {
                self[step.dst].push(item);
            }
        }
    }
    fn rearrange_multi(&mut self, step: &Move) {
        let mut queue: Vec<char> = Vec::with_capacity(step.quantity);
        for _ in 0..step.quantity {
            if let Some(item) = self[step.src].pop() {
                queue.push(item)
            }
        }
        for item in queue.iter().rev() {
            self[step.dst].push(*item);
        }
    }
}

/// This structure represent a move of a specify quantity from `src` to `dst`.
struct Move {
    quantity: usize,
    src: usize,
    dst: usize,
}
impl TryFrom<&String> for Move {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let values: Vec<usize> = value.split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();

        if values.len() != 3 {
            Err(())
        } else {
            Ok(Move {quantity: values[0], src: values[1] - 1, dst: values[2] - 1})
        }
    }
}

/// Reads the diagram from and returns a collection of stacks that represent the stacks of crates.
fn read_diagram(diagram: &Vec<String>) -> Vec<Vec<char>> {
    let mut row_indices: Vec<usize> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for (i, line) in diagram.iter().rev().enumerate() {
        if i == 0 {
            for (j, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    row_indices.push(j);
                    stacks.push(Vec::<char>::new());
                }
            }
        } else {
            for (j, row) in row_indices.iter().enumerate() {
                if let Some(item) = line.get(*row..row + 1) {
                    if let Some(c) = char::from_u32(item.as_bytes()[0] as u32) {
                        if c.is_alphabetic() {
                            stacks[j].push(c);
                        }
                    }
                }
            }
        }
    }
    stacks
}

/// Runs the day 5 program.
pub fn main(path: &String) {
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut diagram: Vec<String> = Vec::new();
            let mut stacks_option: Option<Vec<Vec<char>>> = None;
            let mut stacks_multi_option: Option<Vec<Vec<char>>> = None;

            for line in reader.lines().map_while(|res| res.ok()) {
                if line.is_empty() {
                    stacks_option = Some(read_diagram(&diagram));
                    stacks_multi_option = Some(read_diagram(&diagram));
                } else {
                    match stacks_option {
                        Some(ref mut stacks) => {
                            let step = Move::try_from(&line).unwrap();

                            stacks.rearrange(&step);

                            if let Some(ref mut stacks_multi) = stacks_multi_option {
                                stacks_multi.rearrange_multi(&step)
                            }
                        },
                        None => diagram.push(line),
                    }
                }
            }
            if let Some(stacks) = stacks_option {
                let mut tops = String::new();
                for stack in stacks {
                    tops += &stack.last().unwrap().to_string();
                }
                println!("part 1: {}", tops);
            }
            if let Some(stacks) = stacks_multi_option {
                let mut tops = String::new();
                for stack in stacks {
                    tops += &stack.last().unwrap().to_string();
                }
                println!("part 2: {}", tops);
            }
        },
        Err(err) => println!("error: Could not read file: {}", err),
    }
}

