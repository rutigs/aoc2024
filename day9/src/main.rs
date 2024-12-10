use std::{char, collections::HashSet, fs::File, io::Read, usize};

fn main() {
    let mut file = File::open("../inputs/day9.txt").expect("unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read the file");

    let mut expanded: Vec<String> = Vec::new();
    let empty_string = String::from('.');

    let input = contents.trim().chars().collect::<Vec<char>>();
    for index in (0..input.len()).step_by(2) {
        let idx = index / 2;

        if index == input.len()-1 {
            // special case for the end
            let count_used = &input[index].to_string().parse::<usize>().unwrap();
            let used: Vec<String> = vec![idx.to_string(); *count_used];
            expanded.extend(used);
            break;
        }
        let pair = &input[index..index+2];

        let count_used = pair[0].to_string().parse::<usize>().unwrap();
        let used: Vec<String> = vec![idx.to_string(); count_used]; 
        
        let count_free = pair[1].to_string().parse::<usize>().unwrap();
        let space: Vec<String> = vec![empty_string.clone(); count_free];

        expanded.extend(used);
        expanded.extend(space);
    }
    
    let mut left = 0;
    let mut right = expanded.len()-1;

    // Part 1
    // while left < right {
    //     let right_val = &expanded[right];

    //     if *right_val == empty_string {
    //         right -= 1;
    //         continue;
    //     }

    //     let left_val = &expanded[left];
    //     if *left_val != empty_string {
    //         left += 1;
    //         continue;
    //     }

    //     expanded[left] = right_val.to_string();
    //     expanded[right] = String::from('.');
    // }
    
    let mut swapped: HashSet<String> = HashSet::new();

    // Part 2
    while right > 0 {
        // find start of file
        let right_val = &expanded[right];
        if *right_val == empty_string {
            right -= 1;
            continue;
        }

        // find start of space
        let left_val = &expanded[left];
        if *left_val != empty_string {
            left += 1;
            continue;
        }

        // calculate length of file
        let mut right_len = 1;
        let mut next_right = &expanded[right-1];
        while *next_right == *right_val && right - right_len != 0  {
            right_len += 1;

            if right == 1 && right_len > 0 {
                break;
            }
            next_right = &expanded[right-right_len];
        }

        // calculate length of space
        let mut left_len = 1;
        let mut next_left = &expanded[left+1];
        while *next_left == *left_val && left + left_len < right {
            left_len += 1;
            
            if left + left_len >= right {
                break;
            }
            next_left = &expanded[left+left_len];
        }

        // the space isn't big enough for the file, try the next space
        if left_len < right_len {
            // there is no free space big enough for this file
            if left + left_len >= right - right_len {
                right = right - right_len;
                left = 0;
                continue;
            }
            left = left+left_len;
            continue;
        }

        if swapped.contains(right_val) {
            break;
        } else {
            swapped.insert(right_val.to_string());
        }

        for i in 0..right_len {
            expanded[left+i] = expanded[right-i].clone();
            expanded[right-i] = String::from('.');
        }

        // we've done a swap, so try the next rightwards number and reset left
        println!("Swap: {:?}", expanded);
        println!("Left: {}, Right: {}", left, right);
        left = 0;
        right -= right_len;
        println!("Left: {}, Right: {}", left, right);
        println!("      {},        {}", expanded[left], expanded[right]);
    }

    println!("{:?}", expanded);

    let mut result = 0;
    for (i, num) in expanded.iter().enumerate() {
        if *num == empty_string {
            continue;
        }

        let n = num.parse::<usize>().unwrap();
        result += i * n
    }

    println!("{}", result);
}

