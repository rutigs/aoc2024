use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::Read};

fn main() {
    let mut file = File::open("../inputs/day10.txt").expect("unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read the file");

    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in contents.split("\n").into_iter() {
        if line.is_empty() { continue }

        map.push(line.trim()
            .chars()
            .map(|n| n.to_digit(10).unwrap() as u32)
            .collect());
    }

    // find all 0s
    // push indices of 0s to a queue as a tuple (i_of_0, j_of_0, i_curr_num, j_curr_num)
    // while queue not empty
    //     look at cardinal posn's to current_num
    //         if current_num = 8 and posn's == 9
    //             evaluate if trailhead score inc.
    //             if (0_posn, 9_posn) pair is unique store key->val
    //         if posn = curr_num+1
    //             push (i_of_0, j_of_0, curr_num+1) to queue
    //
    // return count of trailhead->peak
    
    let mut potential_routes: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                potential_routes.push_back((i, j, i, j));
            }
        }
    }

    // Part 1
    // let mut trails: HashSet<(usize, usize, usize, usize)> = HashSet::new();
    // Part 2
    let mut trail_scores: HashMap<(usize, usize), u32> = HashMap::new();

    while let Some(curr_route) = potential_routes.pop_front() {
        let (x, y) = (curr_route.2, curr_route.3);
        let curr_num = map[x][y];

        let up = get_neighbour(&map, x, y, Direction::Up);
        let down = get_neighbour(&map, x, y, Direction::Down);
        let left = get_neighbour(&map, x, y, Direction::Left);
        let right = get_neighbour(&map, x, y, Direction::Right);
        let adj_vals = vec!(up, down, left, right);

        for neighbour in adj_vals {
            match neighbour {
                Some(neighbour_node) => {
                    let val = map[neighbour_node.0][neighbour_node.1];
                    if curr_num == 8 && val == 9 {
                        // Part 1
                        // trails.insert((curr_route.0, curr_route.1, neighbour_node.0, neighbour_node.1));
                        
                        // Part 2
                        let trail_score = match trail_scores.get(&(curr_route.0, curr_route.1)) {
                            Some(c) => *c + 1,
                            None => 1,
                        };
                        trail_scores.insert((curr_route.0, curr_route.1), trail_score);
                    } else if val == curr_num + 1 {
                        potential_routes.push_back((curr_route.0, curr_route.1, neighbour_node.0, neighbour_node.1));
                    }
                },
                None => continue,
            };
        }
    }

    // Part 1
    // let mut trail_scores: HashMap<(usize, usize), u32> = HashMap::new();
    // for trail in trails {
    //     let new_trail_score = match trail_scores.get(&(trail.0, trail.1)) {
    //         Some(c) => *c + 1,
    //         None => 1,
    //     };
    //     trail_scores.insert((trail.0, trail.1), new_trail_score);
    // }

    println!("{}", trail_scores.values().sum::<u32>());
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_neighbour(map: &Vec<Vec<u32>>, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
    return match dir {
        Direction::Up => if x == 0 { None } else { Some((x-1, y)) },
        Direction::Down => if x == map.len()-1 { None } else { Some((x+1, y)) },
        Direction::Left => if y == 0 { None } else { Some((x, y-1)) },
        Direction::Right => if y == map[0].len()-1 { None } else { Some((x, y+1)) },
    }
}
