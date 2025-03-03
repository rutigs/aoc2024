#![allow(dead_code)]

use std::{cell::RefCell, collections::{HashMap, VecDeque}, io::Read, rc::Rc};
use std::fs::File;

struct Game {
    button_a: Button,
    button_b: Button,
    prize: Prize
}

impl Game {
    fn new(button_a: Button, button_b: Button, prize: Prize) -> Self {
        Self {
            button_a,
            button_b,
            prize
        }
    }
}

struct Button {
    x: u64,
    y: u64
}

impl Button {
    fn from(s: &str) -> Self {
        let s: Vec<&str> = s.trim()
            .split_whitespace()
            .skip(2)
            .collect();
        
        let x = &s[0][2..s[0].len()-1];
        let y = &s[1][2..];

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

struct Prize {
    x: u64,
    y: u64
}

impl Prize {
    fn from(s: &str) -> Self {
        let s: Vec<&str> = s.trim()
            .split_whitespace()
            .skip(1)
            .collect();

        let x = &s[0][2..s[0].len()-1];
        let y = &s[1][2..];

        // Self {
        //     x: x.parse::<u64>().unwrap() + 10000000000000,
        //     y: y.parse::<u64>().unwrap() + 10000000000000,
        // }

        // Part 1
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Vertex {
    x: u64,
    y: u64,
}

impl Vertex {
    fn new(x: u64, y: u64) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Edge {
    target: Vertex,
    weight: u64,
}

struct Graph {
    nodes: HashMap<Vertex, Vec<Edge>>,
}

impl Graph {
    fn from(root: Vertex, game: &Game) -> Self {
        let mut nodes: HashMap<Vertex, Vec<Edge>> = HashMap::new();

        // start a queue of nodes to look at 
        let mut queue = VecDeque::from([(root.x, root.y)]);
        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();
            let curr_vertex = Vertex::new(vertex.0, vertex.1);
            if nodes.contains_key(&(curr_vertex)) { continue }

            let mut new_edges = Vec::new();

            // from current node (which starts at 0,0)
            // look at curr + button_a
            // if its <= prize
            //      create edge from curr node
            //      add new_node to queue
            let (a_x, a_y) = (vertex.0 + game.button_a.x, vertex.1 + game.button_a.y);
            let vertex_a = Vertex::new(a_x, a_y);
            if vertex_a.x <= game.prize.x && vertex_a.y <= game.prize.y {
                let edge_a = Edge { target: vertex_a, weight: 3 };
                new_edges.push(edge_a);
                queue.push_back((a_x, a_y));
            }

            // same as above for button a
            let (b_x, b_y) = (vertex.0 + game.button_b.x, vertex.1 + game.button_b.y);
            let vertex_b = Vertex::new(b_x, b_y);
            if vertex_b.x <= game.prize.x && vertex_b.y <= game.prize.y {
                let edge_b = Edge { target: vertex_b, weight: 1 };
                new_edges.push(edge_b);
                queue.push_back((b_x, b_y));
            }

            // add the node to the hashmap regardless of its edges
            // this will make checking if the prize is possible quick before finding min path
            nodes.insert(curr_vertex, new_edges);
        }

        Self {
            nodes
        }
    }

    fn min_path_to_prize(self, game: &Game) -> u64 {
        let prize = Vertex::new(game.prize.x, game.prize.y);
        if !self.nodes.contains_key(&prize) { return 0 }

        let mut distances: HashMap<(u64, u64), Rc<RefCell<GraphNode>>> = HashMap::new();
        let mut unvisited: Vec<Rc<RefCell<GraphNode>>> = Vec::new();

        for n in self.nodes.keys() {
            let mut dist = u64::MAX;
            if n.x == 0 && n.y == 0 { dist = 0 }

            let g = Rc::new(RefCell::new(GraphNode {
                node: (n.x, n.y),
                dist,
                prev_node: None,
            }));
            
            distances.insert((n.x, n.y), Rc::clone(&g));
            unvisited.push(Rc::clone(&g));
        }

        while !unvisited.is_empty() {
            unvisited.sort_by_key(|k| k.borrow().dist);
            let curr_node = unvisited.remove(0);
            // this isn't popping the smallest unvisited item TODO

            let curr_vertex = Vertex{ x: curr_node.borrow().node.0, y: curr_node.borrow().node.1 };

            // Get the current node's details
            let curr_graph_node = distances.get(&(curr_vertex.x, curr_vertex.y)).unwrap();
            let curr_dist = curr_graph_node.borrow().dist;
            // println!("Looking at {},{} with dist={}", curr_vertex.x, curr_vertex.y, curr_dist);
            
            // Get its edges, compare it edges to the cost of the current node and update the
            // shortest path for its targets
            if let Some(curr_edges) = self.nodes.get(&curr_vertex) {
                for edge in curr_edges {
                    let target_graph_node = distances.get_mut(&(edge.target.x, edge.target.y)).unwrap();
                    let target_dist = (*target_graph_node).borrow().dist;
                    if curr_dist + edge.weight < target_dist {
                        (*target_graph_node).borrow_mut().dist = curr_dist + edge.weight;
                        (*target_graph_node).borrow_mut().prev_node = Some((curr_vertex.x, curr_vertex.y));

                        // println!("Updated {:?} to dist={}", (*target_graph_node).borrow().node, (*target_graph_node).borrow().dist);
                    }
                }
            }
        }

        if let Some(prize_val) = distances.get(&(game.prize.x, game.prize.y)) {
            return (*prize_val).borrow().dist;
        }
        0
    }
}

struct GraphNode {
    node: (u64, u64),
    dist: u64,
    prev_node: Option<(u64, u64)>
}

fn main() {
    let mut file = File::open("../inputs/day13.txt").expect("unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read the file");

    let mut button_a = "";
    let mut button_b = "";

    let mut games: Vec<Game> = Vec::new();

    for line in contents.lines() {
        if line.is_empty() { 
            (button_a, button_b) = ("", "");
            continue
        }

        if line.starts_with("Button A") {
            button_a = line;
        } else if line.starts_with("Button B") {
            button_b = line;
        } else if line.starts_with("Prize") {
            games.push(
                Game::new(
                    Button::from(button_a),
                    Button::from(button_b), 
                    Prize::from(line))
            );
        }
    }

    // construct graphs for each game
    // each node will have one incoming and 2 outgoing edges based on button a or button b
    // curr_node(x,y) = last_node(x,y) + button_a/b(x,y)
    // the weight will be 3 for a, 1 for b
    // the prize may or may not be a vertex on the graph
    // find the min path to all prizes

    let mut total_prize_cost = 0;

    for (i, game) in games.iter().enumerate() {
        // println!("Button A: X+{}, Y+{}", game.button_a.x, game.button_a.y);
        // println!("Button B: X+{}, Y+{}", game.button_b.x, game.button_b.y);
        // println!("Prize: X={}, Y={}", game.prize.x, game.prize.y);

        // Part 1
        // let root = Vertex::new(0, 0);
        // let graph = Graph::from(root, &game);
        // total_prize_cost += graph.min_path_to_prize(&game);
        
        // Part 2 - numerical optimization
        println!("Game #{}", i+1);
        total_prize_cost += part2(&game);
    }

    println!("{}", total_prize_cost);
}

fn part2(game: &Game) -> u64 {
    // determinant is the amount the transformation affects the area
    // if determinant == 0, we squish the vectors down a dimension (aka 0 or inf solns) 
    // Button_A X val * A presses + Button_B X val * B presses = Prize X dist
    // Button_A Y val * A presses + Button_B Y val * B presses = Prize Y dist
    // a_x * num_a + b_x * num_b = prize_x
    // a_y * num_a + b_y * num_b = prize_b
    // solve for num_a and num_b
    
    println!("Game start");

    let determinant = match (game.button_a.x * game.button_b.y).checked_sub(game.button_a.y * game.button_b.x) {
        Some(val) => val,
        None => 0,
    };
    if determinant == 0 {
        return 0;
    }

    println!("Game Det: {}", determinant);

    let determinant_a = match (game.prize.x * game.button_b.y).checked_sub(game.prize.y * game.button_b.x) {
        Some(val) => val,
        None => 0,
    };
    let determinant_b = match (game.button_a.x * game.prize.y).checked_sub(game.button_a.y * game.prize.x) {
        Some(val) => val,
        None => 0,
    };
    println!("Det_A: {}", determinant_a);
    println!("Det_B: {}", determinant_b);

    if determinant_a == 0 || determinant_b == 0 {
        return 0;
    }

    // num_a = area_a / det
    // num_a = det_a / det
    let num_a = determinant_a / determinant;
    let num_b = determinant_b / determinant;
    println!("{}", num_a * 3 + num_b);
    num_a * 3 + num_b
}
