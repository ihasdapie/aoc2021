use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;

fn mk_file_reader(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).unwrap();
    BufReader::new(file)
}


fn part1(col: Vec<Vec<i32>>) -> i32 {
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for c in &col {
        let num_zero = c.iter().filter(|&x| *x == 0).count();
        let num_one = c.iter().filter(|&x| *x == 1).count();

        if num_one > num_zero {
            gamma = gamma.clone() + "1";
            epsilon = epsilon.clone() + "0";
        } else {
            gamma = gamma.clone() + "0";
            epsilon = epsilon.clone() + "1";
        }
    }
    
    i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap()
}

fn remove_multiple<T>(source: &mut Vec<T>, idx_to_remove: &Vec<usize>) -> Vec<T> {
    idx_to_remove.iter().copied().map(|i| source.swap_remove(i)).collect()
}


fn part2(diag_msg: Vec<String>, col: Vec<Vec<i32>>) -> i32 {
    let mut remaining_diag_msg = vec![];
    let mut ogr: String;
    let mut csr: String;
    let mut match_idx: HashSet<usize> = HashSet::new();

    for i in 0..col[0].len() {
        remaining_diag_msg.push(i);
    }


    let c1 = col.clone();



    for c in &c1 {
        let num_zero = c.iter().filter(|&x| *x == 0).count();
        let num_one = c.iter().filter(|&x| *x == 1).count();
        let mut cur_match_idx: HashSet<usize> = HashSet::new();


        if (remaining_diag_msg.len() - match_idx.len()) == 1 {
            break;
        }
        
        if num_one > num_zero {
            for (i, x) in c.iter().enumerate() {
                if *x == 0 {
                    match_idx.insert(i);
                    cur_match_idx.insert(i);
                }
            }
        } else {
            for (i, x) in c.iter().enumerate() {
                if *x == 1 {
                    match_idx.insert(i);
                    cur_match_idx.insert(i);
                }
            }
        } 

        let mut match_idx_sorted: Vec<usize> = cur_match_idx.into_iter().collect();
        match_idx_sorted.sort();

        println!("{:?}", match_idx_sorted);

        for (coli, column) in c1.iter().enumerate() {
            for (i, removeable) in match_idx_sorted.iter().enumerate() {
                column.remove(removeable-i);
            }
        }

        cur_match_idx.clear();

        

        println!("{} {} col size", col.len(), col[0].len());

        // we need to remove rows from col as we go along
    }



    remaining_diag_msg.retain(|&i| !match_idx.contains(&i));

    ogr = diag_msg[remaining_diag_msg[0]].clone();
    println!("{}", ogr);

    match_idx.clear();
    remaining_diag_msg.clear();

    for i in 0..col[0].len() {
        remaining_diag_msg.push(i);
    }



    for c in &col.clone() {
        let num_zero = c.iter().filter(|&x| *x == 0).count();
        let num_one = c.iter().filter(|&x| *x == 1).count();

        if (remaining_diag_msg.len() - match_idx.len()) == 1 {
            break;
        }


        if num_one > num_zero {
            for (i, x) in c.iter().enumerate() {
                if *x == 1 {
                    match_idx.insert(i);
                }
            }
        } else {
            for (i, x) in c.iter().enumerate() {
                if *x == 0 {
                    match_idx.insert(i);
                }
            }
        }

    }

    remaining_diag_msg.retain(|&i| !match_idx.contains(&i));

    csr = diag_msg[remaining_diag_msg[0]].clone();
    println!("{}", csr);


    i32::from_str_radix(&csr, 2).unwrap() * i32::from_str_radix(&ogr, 2).unwrap()

}



fn main() {

    let file_path = "dummy.txt";
    let file_reader = mk_file_reader(file_path);

    let mut v = vec![];
    for line in file_reader.lines() {
        let line = line.unwrap();
        v.push(line);
    }

    let msg_length = v[0].len();



    let mut col: Vec<Vec<i32>> = vec![vec![]; msg_length];


    for line in &v {
        for i in 0..msg_length {
            col[i].push(line.chars().nth(i).unwrap().to_digit(10).unwrap() as i32);
        }
    }


    let ans = part1(col.clone());
    let ans = part2(v , col.clone());


    println!("{}", ans);



}
