use d14::{polymerize, read_input};
use std::collections::HashMap;
use std::env;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut lines = read_input(filename).into_iter();

    let sequence: String = lines.next().unwrap();
    let _ = lines.next();

    let rules: HashMap<String, char> =
        HashMap::from_iter(lines.filter(|x| !x.is_empty()).map(|x| {
            let mut parts = x.split(" -> ");
            let left: &str = parts.next().unwrap();
            let right: char = parts.next().unwrap().chars().next().unwrap();
            (left.to_string(), right)
        }));

    println!("for 10 steps: {}", polymerize(&sequence, &rules, 10));
    println!("for 40 steps: {}", polymerize(&sequence, &rules, 40));
}
