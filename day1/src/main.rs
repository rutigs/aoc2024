// Part 1
// use std::{
//     fs::File,
//     io::{BufRead, BufReader},
// };

// fn main() -> std::io::Result<()> {
//     let file = File::open("../inputs/day1.txt")?;
//     let reader = BufReader::new(file);

//     let mut list1: Vec<i32> = Vec::new();
//     let mut list2: Vec<i32> = Vec::new();

//     for line in reader.lines() {
//         let line = line?;
//         let items = line.trim().split_whitespace().collect::<Vec<&str>>();
//         list1.push(items[0].parse::<i32>().unwrap());
//         list2.push(items[1].parse::<i32>().unwrap());
//     }

//     list1.sort();
//     list2.sort();

//     let mut dist: i32 = 0;

//     for i in 0..list1.len() {
//         dist += (list1[i] - list2[i]).abs();
//     }

//     println!("Distance: {}", dist);
//     Ok(())
// }

// Part 2

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("../inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let items = line.trim().split_whitespace().collect::<Vec<&str>>();
        list1.push(items[0].parse::<i32>().unwrap());
        list2.push(items[1].parse::<i32>().unwrap());
    }

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for num in list2 {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
    }

    let mut dist: i32 = 0;
    for num in list1 {
        let count = counts.entry(num).or_insert(0);
        dist += num * *count;
    }

    println!("Distance: {}", dist);

    Ok(())
}
