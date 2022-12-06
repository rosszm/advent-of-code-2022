use std::{fs::File, io::{BufReader, BufRead}};
use itertools::Itertools;


trait Signal {
    /// Returns the number of items processed before a unique sub-set of items of length `size`.
    fn num_processed(&self, size: usize) -> usize;

    /// Returns the number of items processed before the start of a packet.
    fn num_before_packet_start(&self) -> usize {
        self.num_processed(4)
    }

    /// Returns the number of items processed before the start of a message.
    fn num_before_msg_start(&self) -> usize {
        self.num_processed(14)
    }
}
impl Signal for &[u8] {
    fn num_processed(&self, size: usize) -> usize {
        let mut num = size;
        for window in self.windows(size) {
            if window.iter().all_unique() {
                break;
            }
            num += 1;
        }
        num
    }
}

/// Runs the day 6 program.
pub fn main(path: &String) {
    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let bytes = line.as_bytes();

            let before_packet = bytes.num_before_packet_start();
            println!("part 1: {}", before_packet);

            let before_msg = bytes.num_before_msg_start();
            println!("part 2: {}", before_msg);
        }
        Err(err) => println!("error: Could not read file: {}", err),
    }
}