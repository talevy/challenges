#![crate_type = "bin"]


extern crate collections;

use std::collections::hashmap::HashMap;
use std::io;
use std::io::BufferedReader;
use std::default::Default;

pub type NodeIndex = uint;

pub struct CountsTree(pub HashMap<NodeIndex, Vec<NodeIndex>>);

impl FromIterator<(NodeIndex, NodeIndex)> for CountsTree {
    fn from_iter<I: Iterator<(NodeIndex, NodeIndex)>>(mut iter: I) -> CountsTree {
        let (lower, _) = iter.size_hint();
        let mut map = CountsTree(HashMap::with_capacity_and_hasher(lower, Default::default()));
        for (k, v) in iter {
            match map {
                CountsTree(ref mut m) => {
                    m.find_or_insert(k, Vec::new()).push(v);
                    m.find_or_insert(v, Vec::new());
                }
            };
        }

        map
    }
}

impl CountsTree {
    fn get<'a>(&'a self) -> &'a HashMap<NodeIndex, Vec<NodeIndex>> {
        let CountsTree(ref t) = *self;
        t
    }

    fn cut_even_forest(&self, root: NodeIndex) -> (uint, uint) {
        let (even, count) = self.get().get(&root).iter()
            .map(|c| { self.cut_even_forest(*c) })
            .fold((0, 1), |(even, count), (x, y)| (even + x, count + y));
        (even + (count + 1) % 2, count)
    }
}

fn main() {
    let tree: CountsTree = BufferedReader::new(io::stdin())
        .lines()
        .skip(1)
        .map(|line| {
            let ln = line.unwrap().as_slice().trim().split(' ')
                .map(|x| {
                    from_str::<uint>(x).unwrap()
                }).collect::<Vec<uint>>();
            (*ln.get(1), *ln.get(0))
        }).collect::<CountsTree>();

    let (num, _) = tree.cut_even_forest(1);
    print!("{0}", num - 1);
}
