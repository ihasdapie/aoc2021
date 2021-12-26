use std::fs;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn read_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.split('\n').map(|x| x.to_string()).collect()
}




pub fn polymerize(sequence: &str, rules: &HashMap<String, char>, steps: usize) -> i64 {
    // approach: keep track of pairs

    let mut state: HashMap<String, i64> = HashMap::new();

    // initial state
    for c in 0..(sequence.len() - 1) {
        let s = String::from_iter(sequence.chars().skip(c).take(2));
        *state.entry(s).or_insert(0) += 1;
    }


    let mut update: HashMap<String, i64> = HashMap::new();
    // first pass to form the polymer
    for _ in 0..steps {
        for pair in state.keys() {
            // assume all pairs are in the rules 

            // build the new state
            let insertable = rules.get(pair).unwrap();
            let mut new_pair = pair.chars().next().unwrap().to_string() + &insertable.to_string();
            *update.entry(new_pair.clone()).or_insert(0) += state.get(pair).unwrap();
            new_pair = format!("{}{}", *insertable, pair.chars().nth(1).unwrap());
            *update.entry(new_pair).or_insert(0) += state.get(pair).unwrap();
        }
        // and update the state
        state = update.clone(); 
        update.clear();
    } 

    // second pass to get element counts
    let mut counts: HashMap<char, i64> = HashMap::new();
    for (k, v) in state.iter() { 
        *counts.entry(k.chars().nth(1).unwrap()).or_insert(0) += *v;
    }
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max-min
}














