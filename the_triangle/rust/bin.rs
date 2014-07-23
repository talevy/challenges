#![crate_type="bin"]

use std::cmp::max;
use std::io;
use std::iter::Repeat;

fn main() {
    let mut reader = io::stdin();

    // first line tells the height of the triangle
    let height = from_str::<uint>(reader.read_line().unwrap().as_slice().trim()).unwrap();

    // read in triangle as vector (level-order)
    let triangle: Vec<uint> = reader.lines()
        .flat_map(|line| {
            line.unwrap()
            .as_slice()
            .trim()
            .words()
            .map(|x| from_str::<uint>(x).unwrap())
            .collect::<Vec<uint>>().move_iter() // hack copy because of lifetime of `line`
        }).collect();

    // calculate height of each node and store solution as vector s.t.
    // heights[i] = the level of the triangle where element i is found
    let heights: Vec<uint> = range(1, height + 1)
        .flat_map(|x| Repeat::new(x).take(x))
        .collect();

    // allocate solution vector
    let mut maxSum: Vec<uint> = Repeat::new(0u).take(triangle.len()).collect();

    // find index of first leaf
    let last_idx = triangle.len() - height;
    // base case: leaves' sums are set to their value
    for i in range(last_idx, triangle.len()) {
        *maxSum.get_mut(i) = triangle[i];
    }

    // solve recurrance relation.
    for i in range(0, last_idx).rev() {
        *maxSum.get_mut(i) = triangle[i] + max(maxSum[i + heights[i]],
                                               maxSum[i + heights[i] + 1]);
    }

    println!("{}", maxSum[0]);
}
