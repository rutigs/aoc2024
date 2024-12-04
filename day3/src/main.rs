use std::{fs::File, io::BufReader, io::BufRead};
use regex::Regex;

fn main() {
    println!("Hello, world!");
    
    let file = File::open("../inputs/day3.txt");
    let reader = match file {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };

    let mut res: i32 = 0;
    let mut buf: String = String::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return,
        };
        buf.push_str(&line);
    }

    let re = Regex::new(r"do\(\)|don\'t\(\)|mul\([0-9]{1,3}\,[0-9]{1,3}\)").unwrap();
    let mut last_do_item: bool = true;

    for m in re.find_iter(&buf) {
        println!("{}", m.as_str());

        let mut capture = m.as_str();
        last_do_item = match capture {
            "don't()" => false,
            "do()" => true,
            _ if last_do_item => {
                capture = &capture[4..capture.len()-1];
                let mult_args = capture.split(",").collect::<Vec<&str>>();
                res += mult_args[0].parse::<i32>().unwrap() * mult_args[1].parse::<i32>().unwrap();
                true
            },
            _ => last_do_item,
        };
        println!("res: {}", res);

        // part 1 only
        // capture = &capture[4..capture.len()-1];
        // let mult_args = capture.split(",").collect::<Vec<&str>>();

        // res += mult_args[0].parse::<i32>().unwrap() * mult_args[1].parse::<i32>().unwrap();
    }

    println!("{}", res);
}
