use std::collections::HashMap;
use std::env;

use d12::read_input;


#[derive(PartialEq, Eq, Debug)]
struct Cave {
    id: String,
    path: Vec<String>,
    visited: bool,
}


impl Cave {
    fn has_visited(&self, id: &str) -> bool {
        // check if a node has been already visited along the current path the node is on
        for n in &self.path {
            if *n == id {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut cave_sys: HashMap<String, Vec<String>> = HashMap::new();

    let lines = read_input(filename);
    for line in lines.iter().filter(|x| !x.is_empty()) {
        let net = line.split('-').collect::<Vec<&str>>();
        cave_sys
            .entry(net[0].to_string())
            .or_insert_with(|| Vec::new())
            .push(net[1].to_string());
        cave_sys
            .entry(net[1].to_string())
            .or_insert_with(|| Vec::new())
            .push(net[0].to_string());
    }






    println!("----------------------- part 1 -----------------------");

    let mut queue: Vec<Cave> = vec![Cave {
        id: "start".to_string(),
        path: vec!["start".to_string()],
        visited: false,
    }];

    let mut no_paths = 0;

    while !queue.is_empty() {
        // we need to only look at paths that start with "start"
        let current = queue.pop().unwrap();


        if current.id == *"end" {
            println!("Paths: {:?}", current.path);
            no_paths += 1;
            continue;
        }

        for c in cave_sys.get(&current.id).unwrap() {
            if !current.has_visited(c) {
                // can visit large caves any number of times
                queue.push(Cave { id: c.to_string(), path: vec![], visited: false});
                queue.last_mut().unwrap().path.extend(current.path.clone());
                if c.to_lowercase() == *c {
                // can visit small caves at most once
                    queue.last_mut().unwrap().path.push(c.to_string());
                }
            }
        }
    }

    println!("Part 1 Count: {}", no_paths);
    println!("----------------------- part 2 -----------------------");



    let mut queue: Vec<Cave> = vec![Cave {
        id: "start".to_string(),
        path: vec!["start".to_string()],
        visited: false,
    }];

    no_paths = 0;

    while !queue.is_empty() {
        // we need to only look at paths that start with "start"
        let current = queue.pop().unwrap();

        if current.id == *"end" {
            // println!("Paths: {:?}", current.path);
            no_paths += 1;
            continue;
        }

        for c in cave_sys.get(&current.id).unwrap() {
            if !current.has_visited(c) {
                // can visit small caves twice
                queue.push(Cave { id: c.to_string(), path: vec![], visited: current.visited});
                queue.last_mut().unwrap().path.extend(current.path.clone());
                if c.to_lowercase() == *c {
                    queue.last_mut().unwrap().path.push(c.to_string());
                }
            }


            else if current.has_visited(c) && !current.visited && *c != *"start" {
                // cannot visit start >1 time
                queue.push(Cave { id: c.to_string(), path: vec![], visited: true});
                queue.last_mut().unwrap().path.extend(current.path.clone());
            }
        }
    }

    println!("Part 2 Count: {}", no_paths);













}


























