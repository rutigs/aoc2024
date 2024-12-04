use std::{fs::File, io::BufReader, io::BufRead};

fn main() {
    println!("Hello, world!");

    let file = File::open("../inputs/day4.txt");
    let reader = match file {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };

    let mut buf: String = String::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return,
        };
        buf.push_str(&line);
    }


}
