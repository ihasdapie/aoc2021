#![feature (drain_filter)]


const WIDTH: usize = 12;



fn main() {
    // Taken from https://github.com/timvisee/advent-of-code-2021/blob/master/day03b/src/main.rs
    // because I didn't feel like doing this and I thought the iter


    let nums = include_str!("../input.txt").lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..WIDTH)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) /2;
            oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
            oxy.first().copied()
        }).last().unwrap();

    let co2 = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) /2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        }).last().unwrap();

    println!("{}*{}={}", oxy, co2, oxy*co2)
}
