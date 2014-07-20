#![crate_type="bin"]

use std::io::BufferedReader;
use std::io;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());
    for line in reader.lines().take_while(|x| *x != Ok("0.00\n".to_string())) {
        let c = from_str::<f32>(line.unwrap().as_slice().trim()).unwrap();
        let mut curr_n = 0u;
        let mut curr_len = 0.0f32;
        while c > curr_len {
            curr_n += 1u;
            curr_len += 1.0f32/((curr_n as f32) + 1.0f32)
        }
        println!("{} card(s)", curr_n);
    }
}
