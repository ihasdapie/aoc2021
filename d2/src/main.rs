use std::fs::File;
use std::io::{BufRead, BufReader};



fn main() {
    
    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut direction: &str;
    let mut dist: i32;
    let mut depth = 0;
    let mut horz = 0;
    let mut aim = 0;

    let mut cur_iter;


    for (_, line) in reader.lines().enumerate(){
        let line = line.unwrap();
        cur_iter = line.split(" ");
        println!("{}", line);
        direction = cur_iter.next().unwrap();
        dist = cur_iter.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {
                horz += dist;
                depth += aim * dist;
            }
            "down" => {
                aim += dist;
            }
            "up" => {
                aim -= dist;
            }
            &_ => {
                println!("{}", "wtf");
            }
        }

    }
    println!("{}", depth * horz);
}
