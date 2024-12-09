use std::{collections::HashSet, fmt::{self}, fs::File, io::Read};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum TileContents {
    Empty,
    AntiNode,
    Antenna(char),
}

impl fmt::Display for TileContents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let contents = match self {
            TileContents::Empty => '.',
            TileContents::AntiNode => '#',
            TileContents::Antenna(c) => *c,
        };
        write!(f, "{}", contents)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Tile {
    pos: (isize, isize),
    contents: TileContents,
}

impl Tile {
    fn new(c: char, pos: (isize, isize)) -> Self {
        let contents = match c {
            '.' => TileContents::Empty,
            '#' => TileContents::AntiNode,
            c => TileContents::Antenna(c),
        };

        Self {
            pos,
            contents
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({},{})", self.contents, self.pos.0, self.pos.1)
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    antennas: Vec<Tile>,
}

impl Map {
    fn from_str(s: String) -> Self {
        let lines = s.split("\n");

        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut antennas: Vec<Tile> = Vec::new();

        let mut row = 0;
        for line in lines {
            if line.is_empty() {
                continue;
            }

            let mut tile_row: Vec<Tile> = Vec::new();
            let chars: Vec<char> = line.chars().collect();

            for col in 0..chars.len() {
                let new_tile = Tile::new(chars[col], (row, col as isize)); 
                tile_row.push(new_tile);
                
                match new_tile.contents {
                    TileContents::Antenna(_) => antennas.push(new_tile),
                    _ => (),
                }

            }

            row += 1;
            tiles.push(tile_row);
        }

        Self {
            tiles,
            antennas,
        }
    }

    fn unique_antinodes(mut self) -> usize {
        // create antenna pairs
        // calculate antinode vector -> distance from (-b)
        // subtract distance from a, add distance to b
        // for antinode_locns
        //     if on map
        //         mark as antinode

        let mut tmp_antennas = self.antennas.clone();
        let mut antenna_pairs: HashSet<(Tile, Tile) > = HashSet::new();
        let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

        while let Some(antenna) = tmp_antennas.pop() {
            for antenna_2 in tmp_antennas.iter() {
                if antenna_2.contents == antenna.contents {
                    antenna_pairs.insert((antenna.clone(), antenna_2.clone()));

                    // Part 2 - each pair means 2
                    antinodes.insert(antenna.pos);
                    antinodes.insert(antenna_2.pos);
                }
            }
        }

        for (ant_a, ant_b) in antenna_pairs {
            let dist = (ant_b.pos.0 - ant_a.pos.0, ant_b.pos.1 - ant_a.pos.1);
            println!("{} -> {}: {},{}", ant_a, ant_b, dist.0, dist.1);

            // Part 2
            let mut potential_antinode = (ant_a.pos.0 - dist.0, ant_a.pos.1 - dist.1);
            while self.on_map(potential_antinode) {
                if self.valid_antinode(potential_antinode) {
                    antinodes.insert(potential_antinode);
                }
                potential_antinode = (potential_antinode.0 - dist.0, potential_antinode.1 - dist.1);
            }

            potential_antinode = (ant_b.pos.0 + dist.0, ant_b.pos.1 + dist.1);
            while self.on_map(potential_antinode) {
                if self.valid_antinode(potential_antinode) {
                    antinodes.insert(potential_antinode);
                }
                potential_antinode = (potential_antinode.0 + dist.0, potential_antinode.1 + dist.1);
            }

            // Part 1
            // let potential_antinode_1 = (ant_a.pos.0 - dist.0, ant_a.pos.1 - dist.1);
            // let potential_antinode_2 = (ant_b.pos.0 + dist.0, ant_b.pos.1 + dist.1);
            //     
            // println!("Potential @ {},{} for {}", potential_antinode_1.0, potential_antinode_1.1, ant_a.contents);
            // println!("Potential @ {},{} for {}", potential_antinode_2.0, potential_antinode_2.1, ant_a.contents);

            // if self.insert_if_valid_antinode(potential_antinode_1) {
            //     antinodes.insert(potential_antinode_1);
            // }

            // if self.insert_if_valid_antinode(potential_antinode_2) {
            //     antinodes.insert(potential_antinode_2);
            // }
        }

        antinodes.len()
    }

    fn on_map(&self, pos: (isize,isize)) -> bool {
        if pos.0 < 0 || pos.1 < 0 {
            return false;
        }

        (pos.0 < (self.tiles.len() as isize)) && (pos.1 < (self.tiles[0].len() as isize))
    }

    fn valid_antinode(&mut self, pos: (isize,isize)) -> bool {
        if !self.on_map(pos) {
            return false;
        }

        match self.tiles[pos.0 as usize][pos.1 as usize].contents {
            TileContents::AntiNode => false,
            _ => true,
        }
    }
}

fn main() {
    let mut file = File::open("../inputs/day8.txt").expect("unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read the file");

    let map = Map::from_str(contents);
    println!("{}", map.unique_antinodes());
}

