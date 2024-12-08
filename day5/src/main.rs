use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn main() {
    println!("hello world");

    let file = File::open("../inputs/day5.txt");
    let reader = match file {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };

    let mut ordering_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lines: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if line.trim().len() == 5 {
            let nums: Vec<&str> = line.trim().split("|").collect();
            let num1: u32 = nums[0].parse().unwrap();
            let num2: u32 = nums[1].parse().unwrap();

            match ordering_map.get_mut(&num1) {
                Some(values) => values.push(num2),
                None => {
                    ordering_map.insert(num1, vec!(num2));
                }
            }

        } else if line.trim().len() == 0 {
            continue
        } else {
            let nums: Vec<u32> = line.trim().split(",")
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();
            lines.push(nums);
        }
    }

    let mut result = 0;
    for mut line in lines {
        let mut fixed = false;
        for num_idx in 1..line.len() {
            let ordering = ordering_map.get(&line[num_idx]).unwrap();

            let mut curr_idx = num_idx;
            while !valid_ordering(curr_idx, &line, ordering) {
                line.swap(curr_idx, curr_idx-1);
                curr_idx -= 1;
                fixed = true;
            }
        }

        if fixed {
            result += line[line.len() / 2];
        }
    }

    // Part 1
    // for line in lines {
    //     let mut valid = true;

    //     for num_idx in 1..line.len() {
    //         if !valid {
    //              break;
    //         }

    //         // get the ordering map for this number in the list
    //         match ordering_map.get(&line[num_idx]) {
    //             Some(ordering) => {
    //                 // get all the previous numbers in the row, and see if they are specified to be
    //                 // after this number
    //                 let prev_nums = &line[0..num_idx];
    //                 println!("{}'s ordering: {:?}", &line[num_idx], ordering);
    //                 println!("Line: {:?}", line);
    //                 for num in prev_nums {
    //                     if ordering.contains(num) {
    //                         valid = false;
    //                         break;
    //                     }
    //                 }
    //             },
    //             None => continue
    //         };
    //     }

    //     if valid {
    //         println!("{} is middle of {:?}", line[line.len() / 2], line);
    //         result += line[line.len() / 2];
    //     }
    // }

    println!("{}", result);
}

// fn valid_ordering(prev_nums: &[u32], ordering: &Vec<u32>) -> bool {
//     for num in prev_nums {
//         if ordering.contains(num) {
//             return false;
//         }
//     }
//     return true;
// }

fn valid_ordering(curr_idx: usize, nums: &[u32], ordering: &Vec<u32>) -> bool {
    let prev_nums = &nums[0..curr_idx];
    for num in prev_nums {
        if ordering.contains(num) {
            return false;
        }
    }
    return true;
}
