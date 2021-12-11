use std::{env, fs};

const WIDTH: i32 = 10;
const HEIGHT: i32 = 10;
const STEPS: i32 = 100;


fn near(i: i32, j: i32) -> Vec<(i32, i32)> {
    // given 2d array, return the indicies of all the neighbors of the given cell
    let mut res: Vec<(i32, i32)> = vec![];
    
    if j > 0 { // above
        res.push((i, j - 1)); 
    } 
    
    if j < HEIGHT - 1 { // below
        res.push((i, j + 1));
    }

    if i > 0 { // left
        res.push((i - 1, j));
    }

    if i < WIDTH - 1 { // right
        res.push((i + 1, j));
    }

    if i > 0 && j > 0 { // top left
        res.push((i - 1, j - 1));
    }

    if i < WIDTH - 1 && j > 0 { // top right
        res.push((i + 1, j - 1));
    }

    if i > 0 && j < HEIGHT - 1 { // bottom left
        res.push((i - 1, j + 1));
    }

    if i < WIDTH - 1 && j < HEIGHT - 1 { // bottom right
        res.push((i + 1, j + 1));
    }

    res
}



fn flash_step(octopi: &mut Vec<i32>) -> i64 {
    octopi.iter_mut().for_each(|x| *x += 1);
    // keep tracked of flashed
    let mut flashed = vec![false; (WIDTH*HEIGHT) as usize];
    let mut tot_flashes = 0;
    while !flashed.iter().all(|x| *x) && octopi.iter().filter(|&x| *x > 9).count() > 0 {
        // pretty_print(octopi);
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let cur_idx = (j*WIDTH + i) as usize;
                if octopi[cur_idx] > 9 && !flashed[cur_idx] {
                    tot_flashes += 1;
                    flashed[cur_idx] = true;
                    for (x, y) in near(i, j) {
                        let idx = (y * WIDTH + x) as usize;
                        octopi[idx] += 1;
                    }
                    octopi[cur_idx] = 0;
                }
            }
        }
    }

    for (i, v) in flashed.iter().enumerate() {
        if *v {
            octopi[i] = 0;
        }
    }

    tot_flashes
}


fn pretty_print(octopi: &[i32]) {
    for i in 0..(WIDTH*HEIGHT) {
        if i % WIDTH == 0 {
            println!();
        }
        print!("{} ", octopi[i as usize]);
    }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut input = contents.chars().filter(|x| *x != '\n').map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();

    let mut tot_flashes = 0;
    let mut cur_flashes: i64 = 0;
    let mut i = 0;

    pretty_print(&input);

    for _ in 0..STEPS {
        tot_flashes += flash_step(&mut input);
    }

    // part 2
    while cur_flashes != input.len() as i64 {
        i += 1;
        cur_flashes = flash_step(&mut input);
    }

    println!("Part 1 Total flashes: {}", tot_flashes);
    println!("Part 2 First simultaneous flash: {}", i);
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_near() {
        let res = super::near(0, 0);
        assert_eq!(res, vec![(0, 1), (1, 0), (1, 1)]);
        let res = super::near(1, 1);
        assert_eq!(res, vec![(1, 0), (1, 2), (0, 1), (2, 1), (0, 0), (2, 0), (0, 2), (2, 2)]);
        // and so forth...
    }
    #[test]
    fn test_flash() {
        let mut input = vec![1, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1, 1, 1, 1, 1];
        let expected = vec![3, 4, 5, 4, 3, 4, 3, 4, 2, 4, 5, 3, 0, 3, 5, 4, 2, 2, 1, 4, 3, 4, 5, 4, 3];
        super::flash_step(&mut input);
        assert_eq!(input, expected);
    }
}





