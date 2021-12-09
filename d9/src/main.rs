use std::env;
use std::fs;
use d9::*;

fn main() {
    let testfile = env::args().nth(1).unwrap();
    // let input = include_str!(testfile);
    let input = fs::read_to_string(testfile)
        .unwrap()
        .replace('\n', "")
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut floor = d9::Floor::from_heights(input);


    floor.set_lowpoints();
    floor.display();
    floor.calc_risk_sum();
    floor.get_all_basin_sizes();
    println!("basin sizes: {:?}", floor.basin_sizes);
    println!("product of largest 3 basins: {}", &floor.basin_sizes[0..3].iter().product::<u32>());
    println!("risk sum: {}", floor.risk_sum);
}
