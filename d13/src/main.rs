use d13::read_input;
use std::env;

fn pretty_print(grid: &[Vec<i32>], x: usize, y: usize) {
    for i in 0..y {
        for j in 0..x {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = read_input(filename);

    let locs = lines
        .iter()
        .filter(|x| x.contains(',')) // anything without a comma is a fold instruction or blank line
        .map(|x| x.split(','))
        .map(|mut x| {
            (
                x.next().unwrap().trim().parse::<i32>().unwrap(),
                x.next().unwrap().trim().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();

    let instructions = lines
        .iter()
        .filter(|x| !x.contains(',') && !x.is_empty())
        .map(|x| {
            let idx = x.find('=').unwrap();
            vec![
                if x.chars().nth(idx - 1).unwrap() == 'x' {
                    0
                } else {
                    1
                }, // get axis: 0 = x, 1 = y
                x[idx + 1..].trim().parse::<i32>().unwrap(), // get value
            ]
        })
        .collect::<Vec<Vec<i32>>>();

    // get max x and y
    let mut paper_x = locs.iter().map(|x| x.0).max().unwrap() + 1;
    let mut paper_y = locs.iter().map(|x| x.1).max().unwrap() + 1;

    let mut paper = vec![vec![0; (paper_x + 1) as usize]; (paper_y + 1) as usize];

    for (x, y) in locs {
        paper[y as usize][x as usize] = 1;
    }


    let mut count = 0;

    for i in &instructions {
        count = 0;
        let axis = i[0];
        let value = i[1];
        // pretty_print(&paper, paper_x as usize, paper_y as usize);
        println!("axis: {}, row/col: {}", axis, value);
        // fold up for on y (1)
        // fold to left for on x (0)
        // seems to always fold in half
        if axis == 0 {
            // fold to left
            for x in 0..value {
                for y in 0..paper_y {
                    paper[y as usize][x as usize] += paper[y as usize][(2*value-x) as usize];
                    if paper[y as usize][x as usize] > 0 {
                        paper[y as usize][x as usize] = 1;
                    }
                }
            }
        } else {
            // fold up
            for y in 0..value {
                for x in 0..(paper_x + 1) {
                    paper[y as usize][x as usize] += paper[(2*value - y) as usize][x as usize];
                    if paper[y as usize][x as usize] > 0 {
                        paper[y as usize][x as usize] = 1;
                    }
                }
            }
        }

        if axis == 0 {
            paper_x = value;
        } else {
            paper_y = value;
        }
           // for part 1, count the number of visible dots after 1 step

        for x in 0..(paper_x) {
            for y in 0..paper_y {
                if paper[y as usize][x as usize] > 0 {
                    count += 1
                }
            }
        }

        println!("paper size: {},{}, actual: {} {} {} {}", paper_x, paper_y,
            paper[0].len(), paper.len(), paper[1].len(), paper[2].len());



        println!("dot count: {:?}", count);
    }

    pretty_print(&paper, paper_x as usize, paper_y as usize); 
    // just find out the code by inspection after this is printed out
}


























