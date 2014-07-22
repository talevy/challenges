#![crate_type="bin"]

use std::collections::hashmap::{HashMap, HashSet};
use std::collections::{DList, Deque};
use std::io::BufferedReader;
use std::io;

type Author = String;

static ERDOS: &'static str = "Erdos, P.";

enum ErdosDistance {
    SomeDist(uint),
    Infinity
}

struct ErdosProblem {
    author_map: HashMap<Author, HashSet<Author>>,
}

impl ErdosProblem {
    fn get_dist(&self, name: &Author) -> ErdosDistance {
        let mut visited: HashSet<&Author> = HashSet::new();
        visited.insert(name);

        let mut queue: DList<&Author> = DList::new();
        queue.extend(self.author_map.get(name).iter());
        let mut dist: uint = 1u;
        loop {
            let mut nq: DList<&Author> = DList::new();

            for node in queue.move_iter() {
                visited.insert(node);
                if node == &ERDOS.to_string() {
                    return SomeDist(dist);
                }

                for x in self.author_map.get(node).iter() {
                    if !visited.contains(&x) {
                        nq.push_front(x);
                    }
                }
            }

            if nq.is_empty() {
                return Infinity;
            }

            queue = nq;
            dist += 1u;
        }
    }
}

fn main() {
    let mut reader = BufferedReader::new(io::stdin());
    let num_scenarios = from_str::<uint>(reader.read_line().unwrap().as_slice().trim()).unwrap();
    for scenario in range(1, num_scenarios + 1) {
        let mut author_map: HashMap<Author, HashSet<Author>> = HashMap::new();
        let pn: Vec<uint> = reader.read_line()
            .unwrap()
            .as_slice()
            .trim()
            .words()
            .map(|x| from_str::<uint>(x).unwrap())
            .collect();
        let num_papers = pn[0];
        let num_names = pn[1];
        for _ in range(0, num_papers) {
            let line = reader.read_line().unwrap();
            let names: Vec<Author> = line
                .as_slice()
                .trim()
                .split(':')
                .take(1)
                .flat_map(|x| {
                    x.split_str(".,")
                    .map(|s| {
                        if !s.ends_with(".") {
                            s.trim().to_string().append(".")
                        } else {
                            s.trim().to_string()
                        }
                    })
                })
                .collect();

            for name1 in names.iter() {
                for name2 in names.iter().filter(|x| *x != name1) {
                    author_map.find_with_or_insert_with(
                        name1.as_slice().to_string(),
                        name2,
                        |_, old_v, new_val| {
                            old_v.insert(new_val.as_slice().to_string());
                        },
                        |_, new_val| {
                            let mut h = HashSet::new();
                            h.insert(new_val.as_slice().to_string());
                            h
                        });
                }
            }
        }

        let problem = ErdosProblem {
            author_map: author_map,
        };

        println!("Scenario {}", scenario);
        for _ in range(0, num_names) {
            let line = reader.read_line().unwrap();
            let name = line.as_slice().trim().to_string();
            let erdos_num = problem.get_dist(&name);

            print!("{} ", name);
            match erdos_num {
                SomeDist(d) => println!("{}", d),
                Infinity => println!("infinity")
            }
        }
    }
}
