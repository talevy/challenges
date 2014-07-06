#![crate_type="bin"]

#![allow(unused_must_use)]
#![feature(macro_rules)]

use std::collections::hashmap::{HashSet, HashMap};
use std::io::BufferedReader;
use std::io;

static N: uint = 100u;

macro_rules! read_line(
    ($sp:ident, $st:expr) => (
        from_str::<$sp>($st.read_line().unwrap().as_slice().trim()).unwrap()
    );
)

macro_rules! read_line_as_map(
    (($sp:ident, $sp2:ident), $st:expr) => ( // invoke it like `(input_5 SpecialE)`
        $st.read_line().unwrap().as_slice().trim().split(' ')
            .map(|x| {
                let mut kv = x.split(',');
                let k = from_str::<$sp>(kv.next().unwrap()).unwrap();
                let v = from_str::<$sp2>(kv.next().unwrap()).unwrap();
                (k, v)
            }).collect::<HashMap<$sp, $sp2>>()
    );
)

type Square = uint;

struct SnakesAndLadders {
    leaps: HashMap<Square, Square>
}

impl SnakesAndLadders {
    fn next_moves(&self, square: Square) -> Vec<uint> {
        range(square + 1, square + 7).map(|x| {
            if self.leaps.contains_key(&x) {
                *self.leaps.get(&x)
            } else {
                x
            }
        }).collect()
    }

    fn min_rolls_to_win(self) -> uint {
        let mut q = HashSet::<Square>::new();
        q.insert(0u);
        let mut num_rolls = 0u;
        loop {
            let mut nq = HashSet::<Square>::new();
            for curr_square in q.iter() {
                if *curr_square == N {
                    return num_rolls;
                }
                nq.extend(self.next_moves(*curr_square).move_iter());
            }
            q = nq;
            num_rolls += 1;
        }
    }
}

fn main() {
    let mut reader = BufferedReader::new(io::stdin());

    let num_tests = read_line!(uint, reader);
    for _ in range(0, num_tests) {
        // ignore number of snakes and ladders
        reader.read_line();
        let mut snakes_and_ladders = read_line_as_map!((uint, uint), reader);
        snakes_and_ladders.extend(read_line_as_map!((uint, uint), reader).move_iter());
        let sol = SnakesAndLadders {
            leaps: snakes_and_ladders
        }.min_rolls_to_win();

        println!("{0}", sol);
    }
}
