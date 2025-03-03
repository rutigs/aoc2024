#![allow(dead_code)]

use std::{fmt::Display, fs::File, io::Read};
use std::{thread, time};

const X_LEN: i64 = 103;
const Y_LEN: i64 = 101;

struct Vector<T> {
    x: T,
    y: T,
}

struct Robot {
    pos: Vector<i64>,
    velocity: Vector<i64>,
}

impl Robot {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.trim().split_whitespace().collect();
        let pos_nums: Vec<i64> = (&split[0][2..]) // remove the "p="
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        let vec_nums: Vec<i64> = (&split[1][2..]) // remove the "v="
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            pos: Vector {
                x: pos_nums[1],
                y: pos_nums[0],
            },
            velocity: Vector {
                x: vec_nums[1],
                y: vec_nums[0],
            }
        }
    }

    fn tick(&mut self) {
        // self.pos.x = match self.pos.x + self.velocity.x {
        //     new_x if new_x < 0 => X_LEN + new_x,
        //     new_x => new_x % X_LEN,
        // };

        // self.pos.y = match self.pos.y + self.velocity.y {
        //     new_y if new_y < 0 => Y_LEN + new_y,
        //     new_y => new_y % Y_LEN,
        // }
        
        self.pos.x = (self.pos.x + self.velocity.x).rem_euclid(X_LEN);
        self.pos.y = (self.pos.y + self.velocity.y).rem_euclid(Y_LEN);
    }

    // (top left, top right, bottom left, bottom right)
    fn quadrant(&self) -> (i64, i64, i64, i64) {
        let x = self.pos.x;
        let y = self.pos.y;
        
        // tricky off by 1 thing here
        // the middle number is len / 2 + 1, but its zero indexed so no need to +1
        let x_mid = X_LEN / 2;
        let y_mid = Y_LEN / 2;
        match (x,y) {
            (x, y) if x < x_mid && y < y_mid => (1,0,0,0),
            (x, y) if x > x_mid && y < y_mid => (0,1,0,0),
            (x, y) if x < x_mid && y > y_mid => (0,0,1,0),
            (x, y) if x > x_mid && y > y_mid => (0,0,0,1),
            _ => (0,0,0,0),
        }
    }
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Robot Pos=({},{}), Vec=({},{})", self.pos.x, self.pos.y, self.velocity.x, self.velocity.y)
    }
}

fn main() {
    let mut file = File::open("../inputs/day14.txt").expect("unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read the file");

    let mut robots: Vec<Robot> = Vec::new();

    for line in contents.lines() {
        if line.is_empty() { 
            continue;
        }
        
        robots.push(Robot::from(line));
    }

    // let mut quadrants: (i64, i64, i64, i64) = (0,0,0,0);
    // let mut map: Vec<Vec<usize>> = vec![vec![0; Y_LEN as usize]; X_LEN as usize];

    // part 1
    // for mut robot in robots {
    //     println!("Before {}", robot);
    //     for _ in 0..100 {
    //         robot.tick();
    //     }

    //     let robot_quadrant = robot.quadrant();
    //     quadrants.0 += robot_quadrant.0;
    //     quadrants.1 += robot_quadrant.1;
    //     quadrants.2 += robot_quadrant.2;
    //     quadrants.3 += robot_quadrant.3;
    //     println!("After {} w/ Quadrant: {:?}", robot, robot_quadrant);

    //     map[robot.pos.x as usize][robot.pos.y as usize] += 1;
    // }

    // for (i, row) in map.iter().enumerate() {
    //     if i == X_LEN as usize / 2 {
    //         let output = (0..Y_LEN).map(|_| " ").collect::<Vec<_>>().concat();
    //         println!("{}", output);
    //     } else {
    //         let mut output = row.iter().map(|n| n.to_string()).collect::<Vec<_>>();
    //         output[Y_LEN as usize / 2] = " ".to_string();
    //         println!("{}", output.concat());
    //     }
    //     // println!("{:?}", row);
    // }
    // let safety_factor = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
    // println!("{}", safety_factor);
    
    for i in 0..10000 {
        let mut map: Vec<Vec<usize>> = vec![vec![0; Y_LEN as usize]; X_LEN as usize];
        for robot in robots.iter_mut() {
            robot.tick();
            map[robot.pos.x as usize][robot.pos.y as usize] += 1;
        }

        println!("Map #{}", i);
        for row in map {
            // println!("{:?}", row);
            let output = (0..row.len())
                .map(|n| n.clamp(0, 1))
                .map(|n| if n == 0 { "." } else { "#" })
                //.map(|n| n.to_string())
                .collect::<Vec<_>>()
                .concat();
            println!("{}", output);
        }
        println!("");
        println!("");
        println!("");

        thread::sleep(time::Duration::from_millis(500));
        //thread::sleep(time::Duration::from_secs(1));
    }
}
