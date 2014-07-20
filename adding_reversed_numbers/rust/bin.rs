#![crate_type="bin"]

use std::io::BufferedReader;
use std::io;

use std::iter::{AdditiveIterator, FromIterator, Rev};
use std::str::Chars;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());

    for line in reader.lines().skip(1) {
        let tragic_line_rev: String = FromIterator::from_iter::<Rev<Chars>>(
            line.unwrap().as_slice().trim().chars().rev());
        let comedic_sum_str: String = FromIterator::from_iter::<Rev<Chars>>(
            tragic_line_rev
            .as_slice()
            .split(' ')
            .map(|x| from_str::<int>(x).unwrap())
            .sum()
            .to_string()
            .as_slice()
            .chars().rev());
        let comedic_sum = from_str::<int>(comedic_sum_str.as_slice()).unwrap();
        println!("{}", comedic_sum);

        /*
         * Another Solution, I think all are ugly...
         *
        let mut char_vec = line.unwrap().into_bytes();
        char_vec.reverse();
        let mut tragic_sum_vec = String::from_utf8(char_vec)
            .unwrap()
            .as_slice()
            .trim()
            .split(' ')
            .sum()
            .to_string()
            .into_byte();
        tragic_sum_vec.reverse();
        let comedic_sum = from_str::<int>(String::from_utf8(tragic_sum_vec).unwrap().as_slice()).unwrap();
        println!("{}", comedic_sum);
        */
    }
}
