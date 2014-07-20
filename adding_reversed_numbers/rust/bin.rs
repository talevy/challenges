#![crate_type="bin"]

// final version cleaned up with help from: https://github.com/Kimundi

use std::io::BufferedReader;
use std::io;
use std::iter::AdditiveIterator;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());

    for line in reader.lines().skip(1) {
        let tragic_line_rev: String = line.unwrap().as_slice().chars().rev().collect();
        let comedic_sum_str: String = tragic_line_rev.as_slice()
            .words()
            .map(|x| from_str::<int>(x).unwrap())
            .sum()
            .to_string()
            .as_slice()
            .chars()
            .rev()
            .collect();
        let comedic_sum = from_str::<int>(comedic_sum_str.as_slice()).unwrap();
        println!("{}", comedic_sum);
    }
}
