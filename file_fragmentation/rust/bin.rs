#![crate_type="bin"]
#![allow(unused_must_use)]

use std::io::BufferedReader;
use std::io;
use std::iter::AdditiveIterator;

static NEW_LINE: &'static str = "\n";

fn get_file(fragments: &Vec<String>) -> Option<String> {
    // crux: there are N files, each having 2 fragments. this is how we 
    // can find the length of the original file because all files are the
    // same.
    let len_of_file = 2 * fragments.iter().map(|x| x.len()).sum() / fragments.len();
    // because there can be multiple solutions, choose first fragment as
    // start of line.
    let first_half = &fragments[0];
    for next_frag in fragments.iter().skip(1) {
        let possible_file = first_half + *next_frag;
        if possible_file.len() == len_of_file {
            return Some(possible_file);
        }
    }

    None
}

fn main() {
    let mut reader = BufferedReader::new(io::stdin());

    let num_cases: uint = from_str::<uint>(reader.read_line().unwrap().as_slice().trim()).unwrap();

    reader.read_line(); // skip empty line

    for _ in range(0, num_cases) {
        let mut fragments: Vec<String> = Vec::new();
        loop {
            match reader.read_line() {
                Ok(l) => {
                    let fragment = l.as_slice();
                    if fragment == NEW_LINE {
                        // split output for next case by a new blank line
                        println!("");
                        break;
                    } else {
                        fragments.push(fragment.trim().to_string());
                    }
                },
                _ => break
            };
        }

        match get_file(&fragments) {
            Some(f) => println!("{}", f),
            None => ()
        }
    }
}
