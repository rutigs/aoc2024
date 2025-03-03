#![allow(dead_code)]

use std::{fs::File, io::Read};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone)]
enum Position {
    Empty,
    Wall,
    Box,
    Robot,
}

impl Position {
    fn to_string(&self) -> String {
        let string = match self {
            Position::Empty => ".",
            Position::Wall => "#",
            Position::Box => "O",
            Position::Robot => "@",
        };
        string.to_string()
    }
}


fn main() {
    let mut map_file = File::open("../inputs/day15-1.txt").expect("unable to open the file");
    let mut map_string = String::new();
    map_file.read_to_string(&mut map_string).expect("unable to read the file");

    let mut moves_file = File::open("../inputs/day15-2.txt").expect("unable to open the file");
    let mut moves_string = String::new();
    moves_file.read_to_string(&mut moves_string).expect("unable to read the file");

    let mut warehouse: Vec<Vec<Position>> = Vec::new();

    for row in map_string.lines() {
        if row.is_empty() { continue };

        let mut warehouse_row = Vec::new();
        for c in row.chars() {
            warehouse_row.push(match c {
                '#' => Position::Wall,
                'O' => Position::Box,
                '.' => Position::Empty,
                '@' => Position::Robot,
                _ => Position::Empty,
            });
        }
        warehouse.push(warehouse_row);
    }

    println!("Before:");
    for row in warehouse.iter() {
        let row_str: Vec<String> = row.iter()
            .map(|i| i.to_string())
            .collect();
        println!("{}", row_str.concat());   
    }
    println!("");

    let mut robot: (isize, isize) = (0, 0);
    for i in 0..warehouse.len() {
        for j in 0..warehouse[0].len() {
            if warehouse[i][j] == Position::Robot {
                robot = (i as isize, j as isize);
            }
        }
    }

    for turn in moves_string.trim().chars() {
        let movement: (isize, isize) = match turn {
            '<' => (0, -1),
            '>' => (0, 1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => panic!("invalid move!"),
        };

        println!("Robot: {},{}", robot.0, robot.1);
        // println!("Move {}: {},{}", turn, movement.0, movement.1);
        println!("Dir: {}", turn);

        let mut positions = vec![(robot.0 + movement.0, robot.1 + movement.1)];

        let mut curr_pos = Position::Robot;
        let mut curr_coords = robot;

        // if next is empty, move curr there
        // if next is a block, push to stack
        // if next is a wall, break
        while let Some(next_posn) = positions.pop() {
            println!("Next: ({},{})", next_posn.0, next_posn.1);
            if next_posn.0 as usize > warehouse.len() || next_posn.1 as usize > warehouse[0].len() {
                break;
            }

            match warehouse[next_posn.0 as usize][next_posn.0 as usize] {
                Position::Empty => {
                    // TODO something happening here
                    warehouse[next_posn.0 as usize][next_posn.0 as usize] = curr_pos.clone();
                    warehouse[curr_coords.0 as usize][curr_coords.1 as usize] = Position::Empty;
                    // this means the robot moved
                    robot = (robot.0 + movement.0, robot.1 + movement.1);
                },
                Position::Box => {
                    curr_pos = Position::Box;
                    curr_coords = next_posn;
                    positions.push((next_posn.0 + movement.0, next_posn.1 + movement.1));
                },
                // Position::Wall => break 'next,
                _ => break,
            }
        }

        println!("After move");
        pretty_print_map(&warehouse);
    }

    pretty_print_map(&warehouse);
}

fn pretty_print_map(warehouse: &Vec<Vec<Position>>) {
    for row in warehouse.iter() {
        let row_str: Vec<String> = row.iter()
            .map(|i| i.to_string())
            .collect();
        println!("{}", row_str.concat());   
    }
}
