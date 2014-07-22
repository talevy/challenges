#![crate_type="bin"]

#![allow(unused_must_use)]
#![feature(macro_rules)]

use std::cmp::max;
use std::io::stdio::StdReader;
use std::io::BufferedReader;
use std::io;
use std::iter::AdditiveIterator;

macro_rules! read_line(
    ($sp:ident, $st:expr) => (
        from_str::<$sp>($st.read_line().unwrap().as_slice().trim()).unwrap()
        );
    )

static NEW_LINE: &'static str = "\n";

trait SortedInsertable<T> {
    fn insert_sorted(&mut self, val: T);
}

impl<T: Ord> SortedInsertable<T> for Vec<T> {
    fn insert_sorted(&mut self, val: T) {
        let idx = self.iter().position(|x| x > &val);
        match idx {
            Some(i) => self.insert(i, val),
            None => self.push(val)
        };
    }
}

fn read_times(reader: &mut BufferedReader<StdReader>) -> Vec<uint> {
    reader.lines()
        .take_while(|x| *x != Ok(NEW_LINE.to_string()))
        .map(|line| {
            from_str::<uint>(line.unwrap().as_slice().trim()).unwrap()
        }).collect()
}

fn next_to_cross(uncrossed: &mut Vec<uint>, start: bool) -> Option<[uint, ..2]> {
    let first = if start {
        uncrossed.remove(0)
    } else {
        uncrossed.pop()
    };
    let second = if start {
        uncrossed.remove(0)
    } else {
        uncrossed.pop()
    };
    match (first, second) {
        (_, None) => None,
        (None, _) => None,
        (Some(a), Some(b)) => Some([a, b])
    }
}

fn next_to_return(crossed: &mut Vec<uint>) -> Option<uint> {
    crossed.remove(0)
}

fn main() {
    let mut reader = io::stdin();

    let num_cases: uint = read_line!(uint, reader);
    reader.read_line(); // skip line
    for i in range(0, num_cases) {
        if i > 0 { println!(""); } // separate cases by empty line.

        let num_people = read_line!(uint, reader);
        let mut is_start: bool = true;
        let mut strategy: Vec<(Option<uint>, Option<uint>)> = Vec::new();
        let mut uncrossed: Vec<uint> = read_times(&mut reader);
        let mut crossed: Vec<uint> = Vec::new();
        uncrossed.sort();

        while !uncrossed.is_empty() {
            let to_cross: [uint, ..2] = next_to_cross(&mut uncrossed, is_start).unwrap();
            is_start = false;
            strategy.push((Some(to_cross[0]), Some(to_cross[1])));
            for x in to_cross.iter() {
                crossed.insert_sorted(*x);
            }

            if crossed.len() != num_people {
                let to_return: uint = next_to_return(&mut crossed).unwrap();
                strategy.push((Some(to_return), None));
                uncrossed.insert_sorted(to_return);
            }
        }

        let total_time = strategy.iter()
            .map(|&b| {
                match b {
                    (Some(x), Some(y)) => max(x, y),
                    (Some(x), None) => x,
                    _ => 0u
                }
            }).sum();

        println!("{}", total_time);
        for step in strategy.move_iter() {
            match step {
                (Some(x), Some(y)) => println!("{} {}", x, y),
                (Some(x), None) => println!("{}", x),
                _ => ()
            }
        }
    }
}
