#![crate_type="bin"]

use std::io::BufferedReader;
use std::io;

static DISTANCE:f32 = 4.5f32;
static SECONDS_IN_HOUR:f32 = 3600f32;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());
    let mut lines = reader.lines();

    loop {
        let n = match lines.next() {
            Some(Ok(x)) => from_str::<int>(x.as_slice().trim()).unwrap(),
            _ => break
        };

        let min_time = range(0, n).map(|_| {
            let v: Vec<int> = lines.next().unwrap().unwrap()
                .as_slice().trim().words()
                .map(|x| from_str::<int>(x).unwrap())
                .collect();
            let (vi, ti) = ((v[0] as f32), v[1]);
            if ti >= 0 {
                let time = (DISTANCE / (vi / SECONDS_IN_HOUR)) + (ti as f32);
                time.ceil() as int
            } else {
                std::int::MAX
            }
        }).min_by(|x| *x as int);

        match min_time {
            Some(t) => println!("{}", t),
            _ => ()
        }
    }
}
