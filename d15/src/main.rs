use std::env;
use d15;


fn main() {
    // dijkstra impl from https://doc.rust-lang.org/std/collections/binary_heap/index.html
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = d15::read_input(filename);

    let grid1 = lines.iter().filter(|x| !x.is_empty()).map(|line| {
        line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();





    // part 2 involves doing the same as part1 but with our current grid
    // as 1/25th of a 5x5 grid
    
    // fill out sideways
    let mut grid2 = grid1.clone();

    let width = grid2[0].len();

    
    for i in 0..4 { 
        for j in 0..grid2.len() {
            let update = grid2[j].clone();
            grid2[j].extend(update.iter().skip(i*width).take(width)
                .map(
                    |x| {
                        match x {
                            1 => 2,
                            2 => 3,
                            3 => 4,
                            4 => 5,
                            5 => 6,
                            6 => 7,
                            7 => 8,
                            8 => 9,
                            9 => 1,
                            _ => panic!("unexpected value")
                        } 
                    }));
        }
    }



    // fill out top-to-down

    for i in 0..(4*grid2.len()) {
        grid2.push(grid2[i]
            .iter().map(|x| {
                match x {
                    1 => 2,
                    2 => 3,
                    3 => 4,
                    4 => 5,
                    5 => 6,
                    6 => 7,
                    7 => 8,
                    8 => 9,
                    9 => 1,
                    _ => panic!("unexpected value")
                }
            }).collect::<Vec<i32>>());


    }

    // d15::pretty_print_i(&grid2);
    println!("part 1: {}", d15::dijkstra(&grid1, (0,0), (grid1.len()-1, grid1[0].len()-1)));
    println!("part 2: {}", d15::dijkstra(&grid2, (0,0), (grid2.len()-1, grid2[0].len()-1)));




}


























