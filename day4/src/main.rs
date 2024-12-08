use std::{fs::File, io::BufReader, io::BufRead};

fn main() {
    let file = File::open("../inputs/day4.txt");
    let reader = match file {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };

    let mut word_search: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return,
        };

        word_search.push(line.chars().collect());
    }

    let mut a_indexes: Vec<(usize, usize)> = Vec::new();
    for row in 0..word_search.len() {
        for col in 0..word_search[row].len() {
            if word_search[row][col] == 'A' {
                a_indexes.push((row, col));
            }
        }
    }

    let mut res = 0;
    a_indexes.into_iter().for_each(|a_index: (usize, usize)| {
        res += if check_xmases(a_index, word_search.clone()) { 1 } else { 0 };
    });

    // Part 1
    // let mut x_indexes: Vec<(usize, usize)> = Vec::new();

    // for row in 0..word_search.len() {
    //     for col in 0..word_search[row].len() {
    //         if word_search[row][col] == 'X' {
    //             x_indexes.push((row, col));
    //         }            
    //     }
    // }

    // let mut res = 0;
    // x_indexes.into_iter().for_each(|x_index: (usize, usize)| {
    //     res += check_adjacent_xmas(x_index, word_search.clone());
    // });

    println!("{}", res);
}

fn check_xmases(a_index: (usize, usize), word_search: Vec<Vec<char>>) -> bool {
    let (x, y) = a_index;
    println!("Checking X index: {}-{}", x, y);

    let x_idx = x as i32;
    let y_idx = y as i32;

    let num_rows = word_search.len() as i32;
    let num_cols = word_search[0].len() as i32;

    let valid_strs = vec!("SM".to_string(), "MS".to_string());

    // check bounds for xmas 
    if y_idx+1 < num_cols && y_idx-1 >=0 && x_idx+1 < num_rows && x_idx-1 >= 0 {
        // check top left then the rest should be fixed
        let top_down_diag = word_search[x-1][y-1].to_string() + &word_search[x+1][y+1].to_string();
        let other_diag = word_search[x+1][y-1].to_string() + &word_search[x-1][y+1].to_string();

        if valid_strs.contains(&top_down_diag) && valid_strs.contains(&other_diag) {
            return true
        }
    }

    return false
}

// Part 1
// fn check_adjacent_xmas(x_index: (usize, usize), word_search: Vec<Vec<char>>) -> usize {
//     let (x, y) = x_index;
//     println!("Checking X index: {}-{}", x, y);
// 
//     let x_idx = x as i32;
//     let y_idx = y as i32;
// 
//     let num_rows = word_search.len() as i32;
//     let num_cols = word_search[0].len() as i32;
// 
//     let mut seen: usize = 0;
// 
//     // forward cases
//     if y_idx+3 < num_cols {
//         // up right
//         if x_idx-3 >= 0 {
//             if word_search[x-1][y+1] == 'M' && word_search[x-2][y+2] == 'A' && word_search[x-3][y+3] == 'S' {
//                 seen += 1;
//             }
//         }
// 
//         // down right
//         if x_idx+3 < num_rows {
//             if word_search[x+1][y+1] == 'M' && word_search[x+2][y+2] == 'A' && word_search[x+3][y+3] == 'S' {
//                 seen += 1;
//             }
//         }
// 
//         // general forwards
//         if word_search[x][y+1] == 'M' && word_search[x][y+2] == 'A' && word_search[x][y+3] == 'S'{
//             seen += 1;
//         }
//     }
// 
//     // backward
//     if y_idx-3 >= 0 {
//         // up left
//         if x_idx-3 >= 0 {
//             if word_search[x-1][y-1] == 'M' && word_search[x-2][y-2] == 'A' && word_search[x-3][y-3] == 'S' {
//                 seen += 1;
//             }
//         }
// 
//         // down left
//         if x_idx+3 < num_rows {
//             if word_search[x+1][y-1] == 'M' && word_search[x+2][y-2] == 'A' && word_search[x+3][y-3] == 'S' {
//                 seen += 1;
//             }       
//         }
// 
//         // general backwards
//         if word_search[x][y-1] == 'M' && word_search[x][y-2] == 'A' && word_search[x][y-3] == 'S'{
//             seen += 1;
//         }
//     }
// 
//     // straight up
//     if x_idx-3 >= 0 {
//         if word_search[x-1][y] == 'M' && word_search[x-2][y] == 'A' && word_search[x-3][y] == 'S' {
//             seen += 1;
//         }
//     }
// 
//     // straight down
//     if x_idx+3 < num_rows {
//         if word_search[x+1][y] == 'M' && word_search[x+2][y] == 'A' && word_search[x+3][y] == 'S' {
//             seen += 1;
//         }
//     }
// 
//     return seen
// }

