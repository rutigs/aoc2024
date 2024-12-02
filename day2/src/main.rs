use std::{fs::File, io::BufReader, io::BufRead};

fn main() {
    let file = File::open("../inputs/day2.txt");
    let reader = match file {
        Ok(f) => BufReader::new(f),
        Err(e) => {
            println!("error: {}", e);
            return;
        },
    };

    let mut result: i32 = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("error: {}", e);
                break;
            },
        };

        // Part 2
        let is_ok = |nums: Vec<&str>| -> bool {
            let num1 = nums[0].parse::<i32>().unwrap();
            let num2 = nums[1].parse::<i32>().unwrap();
            let descending = num1 > num2;

            let mut curr: i32 = num1.clone();
            let mut ok = true;
            for next in nums.iter().skip(1) {
                let next = next.parse::<i32>().unwrap();
                if (descending && curr < next) || (!descending && curr > next) {
                    ok = false;
                    break;
                }

                let diff = (curr - next).abs();
                if  diff < 1 || diff > 3 {
                    ok = false;
                    break;
                }
                curr = next;
            }

            ok 
        };

        let items = line.trim().split_whitespace().collect::<Vec<&str>>();
        for x in 0..items.len() {
            let mut nums = items.clone();
            nums.remove(x);
            if is_ok(nums) {
                result += 1;
                break;
            }
        }

        // Part 1
        // let num1 = items[0].parse::<i32>().unwrap();
        // let num2 = items[1].parse::<i32>().unwrap();
        // let descending = num1 > num2;

        // let mut curr: i32 = num1.clone();
        // let mut ok = true;
        // for next in items.iter().skip(1) {
        //     let next = next.parse::<i32>().unwrap();
        //     if (descending && curr < next) || (!descending && curr > next) {
        //         ok = false;
        //         break;
        //     }

        //     let diff = (curr - next).abs();
        //     if  diff < 1 || diff > 3 {
        //         ok = false;
        //         break;
        //     }
        //     curr = next;
        // }

        // if ok {
        //     result += 1;
        // }
    }

    println!("Result: {}", result);        
}
