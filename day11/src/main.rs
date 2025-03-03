use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut file = File::open("../inputs/day11.txt").expect("unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read the file");

    let stones: Vec<u64> = contents
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .into_iter()
        .collect();

    // stones = part1(stones);
    let stone_count = part2(stones);

    println!("{}", stone_count);
}

// fn part1(stone_list: Vec<u64>) -> Vec<u64> {
//     let mut stones = stone_list.clone();
//     for _blink in 0..25 {
//         let mut new_list: Vec<u64> = Vec::new();
//         for item in stones.iter() {
//             match item {
//                 0 => new_list.push(1),
//                 _ => {
//                     let digits = item.to_string();
//                     if digits.len() % 2 == 1 {
//                         new_list.push(item * 2024)
//                     } else {
//                         let (left, right) = digits.split_at(digits.len() / 2);
//                         new_list.push(left.parse::<u64>().unwrap());
//                         new_list.push(right.parse::<u64>().unwrap());
//                     }
//                 },
//             }
//         }
//         stones = new_list;
//     }
//     stones
// }

fn part2(stone_list: Vec<u64>) -> usize {
    let mut stone_counts = HashMap::new();
    for s in stone_list {
        stone_counts.insert(s, 1);
    }

    for _blink in 0..75 {
        let mut new_counts: HashMap<u64, usize> = HashMap::new();
        for (s, count) in stone_counts {
            if s == 0 {
                *new_counts.entry(1).or_default() += count
            } else {
                let digits = s.to_string();
                if digits.len() % 2 == 1 {
                    *new_counts.entry(s * 2024).or_default() += count;
                } else {
                    let (left, right) = digits.split_at(digits.len() / 2);
                    let left_num = left.parse::<u64>().unwrap();
                    let right_num = right.parse::<u64>().unwrap();

                    *new_counts.entry(left_num).or_default() += count;
                    *new_counts.entry(right_num).or_default() += count;
                }
            }
        }
        stone_counts = new_counts;
    }

    stone_counts.values().sum()
}
