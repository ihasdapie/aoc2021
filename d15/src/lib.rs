use std::fs;
use std::collections::HashMap;
use std::collections::BinaryHeap;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    cost: i32,
    pos: (usize, usize),
}


impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


pub fn read_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.split('\n').map(|x| x.to_string()).collect()
}

/// Dijkstra's algorithm on a grid where neighbours are the top, right, bottom and left
/// if no path is found, return -1
pub fn dijkstra(grid: &[Vec<i32>], start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut distance = HashMap::new();
    
    // starting position
    heap.push(Node{cost: 0, pos: start });
    distance.insert(start , 0);

    while let Some(Node{cost, pos}) = heap.pop() {
        if pos == end {
            return distance[&pos];
        }

        if cost > distance[&pos] {
            continue;
        }
        for neighbour in direct_neighbours(pos.0, pos.1, grid.len(), grid[0].len()) {
            // see if there's a lower cost going thru the neighbour node
            let next = Node{ cost: cost + grid[neighbour.1][neighbour.0], pos: neighbour };
            
            distance.entry(next.pos).or_insert(i32::MAX);
            if next.cost < distance[&next.pos] {
                distance.insert(next.pos, next.cost);
                heap.push(next);
            }
        }
    }
    -1 // since we're on an open grid, there should never not be a path
}

pub fn direct_neighbours(x: usize, y: usize, xsize: usize, ysize: usize) -> Vec<(usize, usize)> {
    // returns neighbour coordinates of a given coordinate
    // where neighbours can be directly above, below, left or right

    let mut neighbours = Vec::new();

    
    if x > 0 {
        neighbours.push((x-1, y));
    }

    if x < xsize-1 {
        neighbours.push((x+1, y));
    }

    if y > 0 {
        neighbours.push((x, y-1));
    }

    if y < ysize-1 {
        neighbours.push((x, y+1));
    }

    neighbours
}




pub fn pretty_print_u(input: &[Vec<u32>]) {
    for row in input {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}



pub fn pretty_print_i(input: &[Vec<i32>]) {
    for row in input {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}









