#![crate_type="bin"]

use std::io::BufferedReader;
use std::io;
use std::iter::AdditiveIterator;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());

    let sum = reader.read_line()
        .unwrap()
        .as_slice()
        .trim()
        .splitn(' ', 2)
        .map(|x| from_str::<int>(x).unwrap())
        .sum();

    println!("{}", sum);
}
