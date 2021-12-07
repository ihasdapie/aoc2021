

const SIMULATION_TIME: u64 = 256;


fn main() {
    let input = include_str!("../input.txt").strip_suffix('\n').unwrap();

    let fish = input.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut fish_hist: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];


    // set initial states

    for i in 0..7 {
        fish_hist[i] = fish.iter().filter(|&x| *x == i as i64).count() as u64;
    }

    println!("init: {:?}", fish_hist);
    

    for i in 0..SIMULATION_TIME {
        let reproduce = fish_hist[0];

        fish_hist[0..7].rotate_left(1);
        fish_hist[6] += fish_hist[7];
        fish_hist[7] = fish_hist[8];
        fish_hist[8] = reproduce;
    }


    





    println!("init: {:?}", fish_hist);
    println!("result: {}", fish_hist.iter().sum::<u64>());



    
}
