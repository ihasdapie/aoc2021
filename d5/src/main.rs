use ndarray::Array2;
use std::cmp::max;


fn main() {
    let input = include_str!("../input.txt").split('\n').collect::<Vec<&str>>();
    let mut x1s: Vec<u32> = Vec::new();
    let mut y1s: Vec<u32> = Vec::new();
    let mut x2s: Vec<u32> = Vec::new();
    let mut y2s: Vec<u32> = Vec::new();


    for line in input.iter() {
        if line.is_empty() {
            break;
        }
        let line_split = line.split(" -> ").collect::<Vec<&str>>();
        let mut coord_1 = line_split[0].split(',');
        x1s.push(coord_1.next().unwrap().parse::<u32>().unwrap());
        y1s.push(coord_1.next().unwrap().parse::<u32>().unwrap());
        let mut coord_2 = line_split[1].split(',');
        x2s.push(coord_2.next().unwrap().parse::<u32>().unwrap());
        y2s.push(coord_2.next().unwrap().parse::<u32>().unwrap());
    }

    let xsize = max(x1s.iter().max(), x2s.iter().max()).unwrap();
    let ysize = max(y1s.iter().max(), y2s.iter().max()).unwrap();


    
    let mut depth_chart = Array2::<u32>::zeros(((*xsize + 1) as usize, (*ysize + 1) as usize));
    for (i, x) in x1s.iter().enumerate() {
        let (x1, y1, x2, y2) = ((*x), y1s[i], x2s[i], y2s[i]);
        // println!("{} {} {} {}", x1, y1, x2, y2);

        // for part1, only consider horz or vertical lines
        /* if !((x1 == x2) | (y1 == y2)) {
            continue;
        } */

        let mut dy = y2 as i32 - y1 as i32;
        let mut dx = x2 as i32 - x1 as i32;

        let length = max(dy.abs(), dx.abs());
        dy /= length;
        dx /= length;


        for i in 0..(length+1) {
            let y: i32 = y1 as i32 + i * dy ;
            let x: i32 = x1 as i32 + i * dx ;
            depth_chart[(y as usize, x as usize)] += 1;

        }

    }

    // count the number of times a point is touched by more than one line
    let mut count = 0;

    for i in 0..depth_chart.shape()[0] {
        for j in 0..depth_chart.shape()[1] {
            if depth_chart[[i, j]] > 1 {
                count += 1;
            }
        }
    }

    println!("{:?}", depth_chart);
    println!(">2 line touch: {}", count);






}
