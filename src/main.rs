use std::fs::File;
use std::io::{prelude::*, BufReader};
use cistemrs::stem;
use std::time::Instant;


fn main() {
    let now = Instant::now();
    let file = File::open("data/de_wordlist_simple.txt").unwrap();
    let reader = BufReader::new(file);

    let mut i = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let word = line.trim();
        let stem = stem(word, false);
        i += stem.len();
    }
    println!("{}", i);
    println!("Elapsed time: {:.3?}", now.elapsed());
}