use std::cmp::min;
fn main() {
    let mut input = include_str!("../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    input.sort_unstable();


    // this is simply the median of all the initial crab positions
    let mut median = 0;
    let pos = ((input.len()/2) as i64).abs() as usize;
    if input.len() % 2 == 0 {
        println!("{}", input.len());
        median = (input[pos] + input[pos-1]) / 2;

    } else {
        median = input[pos];
    }



    // is there a better way to do this?
    println!("num: {}", median);

    // get fuel cost, part1
    let fuel_cost = input.iter().map(|x| (x-median).abs()).sum::<i64>();
    println!("part 1 fuel cost: {}", fuel_cost);

    // get fuel cost, part2
    // we can get the median by taking cost (with gauss's formula), $cost = d(d+1)/2$
    // and then we can take the derivative of that
    // to observe the optimum value is the mean of the distances, +/- 0.5
    
    let input_sum: i64 = input.iter().sum();

    let part2_mean: f64 = (input_sum as f64)/input.iter().len() as f64;
    println!("part 2 mean: {}", part2_mean);

    let part2_fuelcost = min (
        input.iter().map(|x| {
            let dist = ((x-(part2_mean-0.5) as i64) as i64).abs();
            dist * (1 + dist) / 2
        }).sum::<i64>(),

        input.iter().map(|x| {
            let dist = ((x-(part2_mean+0.5) as i64) as i64).abs();
            dist * (1 + dist) / 2
        }).sum::<i64>()

    );


    println!("part 2 fuel cost: {}", part2_fuelcost);


}
