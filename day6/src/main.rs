use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("../inputs/day6.txt");
    let reader = match file {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };

    let mut patrol_map: Vec<Vec<char>> = Vec::new();

    let mut curr_guard: char = '^';
    let mut guard_locn: (i32, i32) = (-1, -1);
    let mut row_idx: i32 = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        patrol_map.push(line.chars().collect());
        if line.contains(curr_guard) {
            let colm_idx = line.find(curr_guard).unwrap();
            guard_locn = (row_idx, colm_idx.try_into().unwrap());
        }
        row_idx += 1;
     }

    // println!("guard starts at {}-{}", guard_locn.0, guard_locn.1);

    println!("Map dims {}x{}", patrol_map.len(), patrol_map[0].len());

    // Part 1
    let dir_vectors: HashMap<char, (i32, i32)> = HashMap::from([
        ('^', (-1, 0)),
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('v', (1, 0))
    ]);

    let mut positions = 1;
    patrol_map[guard_locn.0 as usize][guard_locn.1 as usize] = 'X';

    while is_on_map(guard_locn, patrol_map.len(), patrol_map[0].len()) {
        if patrol_map[guard_locn.0 as usize][guard_locn.1 as usize] == '.' {
            patrol_map[guard_locn.0 as usize][guard_locn.1 as usize] = 'X';
            positions += 1;
        }

        let peek = (guard_locn.0 + dir_vectors[&curr_guard].0, guard_locn.1 + dir_vectors[&curr_guard].1);
        if !is_on_map(peek, patrol_map.len(), patrol_map[0].len()) {
            println!("peek is off map {},{}", peek.0, peek.1);
            break;
        }

        println!("curr guard   = {}", curr_guard);
        println!("current locn = {},{}", guard_locn.0, guard_locn.1);
        println!("current peak = {},{}", peek.0, peek.1);

        curr_guard = match patrol_map[peek.0 as usize][peek.1 as usize] {
            '#' => match curr_guard {
                '^' => '>',
                '<' => '^',
                '>' => 'v',
                'v' => '<',
                _ => panic!("invalid {}", curr_guard),
            },
            _ => curr_guard,
        };

        guard_locn = (guard_locn.0 + dir_vectors[&curr_guard].0, guard_locn.1 + dir_vectors[&curr_guard].1);
    }

    println!("{}", positions);
}

fn is_on_map(locn: (i32, i32), rows: usize, cols: usize) -> bool {
    return (locn.0 >= 0) && (locn.0 < rows as i32) & (locn.1 >= 0) && (locn.1 < cols as i32)
}
