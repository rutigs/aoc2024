use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut file = File::open("../inputs/day12.txt").expect("unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read the file");

    let mut garden_map: Vec<Vec<char>> = Vec::new();

    for line in contents.split("\n").into_iter() {
        if line.is_empty() { continue }

        garden_map.push(line.chars().collect());
    }

    // println!("{}", part1(&garden_map));
    println!("{}", part2(&garden_map))
}


fn part2(garden_map: &Vec<Vec<char>>) -> usize { 
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut areas_and_sides: Vec<(usize, usize)> = Vec::new();

    while visited.len() < garden_map.len() * garden_map[0].len() {
        let mut node = (0, 0);
        for i in 0..garden_map.len() {
            for j in 0..garden_map[0].len() {
                if !visited.contains(&(i,j)) {
                    node = (i,j);
                }
            }
        }
        visited.insert(node);

        let (area, sides, new_visits) = bfs_2(node, garden_map);
        println!("{}: area={}, sides={}", garden_map[node.0][node.1], area, sides);
        areas_and_sides.push((area, sides));
        for n in new_visits {
            visited.insert(n);
        }
    }

    let mut total_cost = 0;
    for (area, sides) in areas_and_sides {
        total_cost += area * sides;
    }
    total_cost
}

fn bfs_2(node: (usize, usize), garden_map: &Vec<Vec<char>>) -> (usize, usize, Vec<(usize, usize)>) {
    let mut queue = Vec::from([(node)]);
 
    let mut area = 0;
    let mut new_visits: HashSet<(usize, usize)> = HashSet::new();

    let mut sides = 0;

    while let Some(item) = queue.pop() {
        let node = item;
        if new_visits.contains(&node) { 
            continue;
        } else {
            new_visits.insert(node);
            area += 1;
        }

        let (i,j) = node;
        let plant_type = garden_map[i][j];
        // println!("{plant_type} at {i},{j}"); 

        let mut neighbour_above = true;
        let mut neighbour_below = true;
        let mut neighbour_left = true;
        let mut neighbour_right = true;

        if i == 0 || garden_map[i-1][j] != plant_type {
            neighbour_above = false;
        } else {
            queue.push((i-1, j));
        }

        if i == garden_map.len() - 1 || garden_map[i+1][j] != plant_type {
            neighbour_below = false;
        } else {
            queue.push((i+1, j));
        }

        if j == 0 || garden_map[i][j-1] != plant_type {
            neighbour_left = false;
        } else {
            queue.push((i, j-1));
        }

        if j == garden_map[0].len() - 1 || garden_map[i][j+1] != plant_type {
            neighbour_right = false;
        } else {
            queue.push((i, j+1));
        }

        // find the corners because corners = sides

        if neighbour_above == false {
            if neighbour_left == false {
                println!("TL Corner at {},{} for {}", i, j, plant_type);
                sides += 1;
            } 

            if neighbour_right == false {
                println!("TR Corner at {},{} for {}", i, j, plant_type);
                sides += 1;
            }
        }

        if neighbour_below == false {
            if neighbour_left == false {
                println!("BL Corner at {},{} for {}", i, j, plant_type);
                sides += 1;
            }

            if neighbour_right == false {
                println!("BR Corner at {},{} for {}", i, j, plant_type);
                sides += 1;
            }
        }

        // look at concave corners
        // e.g.
        // X O
        // X X - this is 6 sides
        //
        // O X O 
        // X X X
        // O X O - this is 12
        if neighbour_above {
            if neighbour_left && garden_map[i-1][j-1] != plant_type {
                println!("TL I-Corner at {},{} for {}", i, j, plant_type);
                sides += 1;
            }

            if neighbour_right && garden_map[i-1][j+1] != plant_type {
                println!("TR I-Corner at {},{} for {}", i, j, plant_type);
                sides += 1;
            }
        }

        if neighbour_below {
            if neighbour_left && garden_map[i+1][j-1] != plant_type {
                println!("BL I-Corner at {},{} for {}", i, j, plant_type);
                sides += 1;
            }

            if neighbour_right && garden_map[i+1][j+1] != plant_type {
                println!("BR I-Corner at {},{} for {}", i, j, plant_type);
                sides += 1;
            }
        }
    }

    let visits_vec: Vec<(usize, usize)> = new_visits.iter().cloned().collect();
    (area,sides,visits_vec)
}

// fn part1(garden_map: &Vec<Vec<char>>) -> usize {
//     let mut visited: HashSet<(usize, usize)> = HashSet::new();
//     let mut areas_and_perims: Vec<(usize, usize)> = Vec::new();
// 
//     // keep a set of visited to make sure we don't visit a node more than once
//     // while visited.len() != garden_map.len()
//     //    go to the first node you haven't visited
//     //    do a bfs for that char capturing area and perimeter along the way
// 
//     // continue until every node has been visited
//     while visited.len() < garden_map.len() * garden_map[0].len() {
//         // find an unvisited node
//         let mut node = (0,0);
//         for i in 0..garden_map.len() {
//             for j in 0..garden_map[0].len() {
//                 if !visited.contains(&(i,j)) { 
//                     node = (i,j)
//                 }
//             }
//         }
//         visited.insert(node);
// 
//         let (area, perim, new_visits) = bfs(node, garden_map);
//         areas_and_perims.push((area,perim));
//         for n in new_visits {
//             visited.insert(n);
//         }
//     }
//   
//     let mut total_cost = 0;
//     for (area, perim) in areas_and_perims {
//         total_cost += area * perim;
//     }
// 
//     total_cost
// }
// 
// fn bfs(node: (usize, usize), garden_map: &Vec<Vec<char>>) -> (usize, usize, Vec<(usize, usize)>) {
//     let mut queue = VecDeque::from([(node)]);
// 
//     let mut area = 0;
//     let mut perimeter = 0;
//     let mut new_visits: HashSet<(usize, usize)> = HashSet::new();
// 
//     while let Some(node) = queue.pop_front() {
//         if new_visits.contains(&node) { 
//             continue;
//         } else {
//             new_visits.insert(node);
//         }
// 
//         let (i,j) = node;
//         let plant_type = garden_map[i][j];
// 
//         area += 1;
//         new_visits.insert(node);
// 
//         // up
//         if i == 0 || garden_map[i-1][j] != plant_type {
//             perimeter += 1;
//         } else {
//             queue.push_back((i-1,j));
//         }
// 
//         // down
//         if i == garden_map.len()-1 || garden_map[i+1][j] != plant_type {
//             perimeter += 1;
//         } else {
//             queue.push_back((i+1,j));
//         }
// 
//         // left
//         if j == 0 || garden_map[i][j-1] != plant_type {
//             perimeter += 1;
//         } else {
//             queue.push_back((i,j-1));
//         }
// 
//         // right
//         if j == garden_map[0].len()-1 || garden_map[i][j+1] != plant_type {
//             perimeter += 1;
//         } else {
//             queue.push_back((i,j+1));
//         }
//     }
// 
//     let visits_vec: Vec<(usize, usize)> = new_visits.iter().cloned().collect();
//     (area, perimeter, visits_vec)
// }

