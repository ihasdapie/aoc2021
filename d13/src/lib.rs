use std::fs;

pub fn read_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.split('\n').map(|x| x.to_string()).collect()
}
