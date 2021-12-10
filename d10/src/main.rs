use std::{env, fs};
use std::collections::HashMap;


fn part1(input: &str, err_score_table: HashMap<char, i32>, open_close_map: HashMap<char, char>, opening: Vec<char>) -> i32 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let mut scores: Vec<i32> = vec![];
    for (i, case) in lines.iter().enumerate() {
        let mut stack: Vec<char> = vec![];
        let mut score = 0;
        // println!("---------------------{}---------------", i);
        let count = 0;
        for c in case.chars() { 
            if opening.contains(&c) {
                stack.push(c);
            } else {
                let popped_open = stack.pop().unwrap();
                let expected_open = open_close_map.get(&c).unwrap();
                println!("{} {} {}", expected_open, popped_open, c);
                if *expected_open != popped_open {
                    score += err_score_table.get(&c).unwrap();
                    if count == 0 {
                        // we only care about the first error
                        scores.push(score);
                    }
                }
            }
        }
    }
    println!("Scores: {:?}", scores);
    scores.iter().sum::<i32>()
}

fn part2(input: &str, autocomplete_score_table: HashMap<char, i64>, open_close_map: HashMap<char, char>, close_open_map: HashMap<char, char>, opening: Vec<char>) -> i64 {
    // idea: the unmatched opening brackets are the ones left in the stack
    // so we can just have a flag to check if we have a balanced sequence or not
    // and then if not, build the fix string from the remaining stack
    let lines = input.split('\n').collect::<Vec<_>>();
    let mut fixes: Vec<String> = vec![];
    for (i, case) in lines.iter().filter(|x| !x.is_empty()).enumerate() {
        let mut stack: Vec<char> = vec![];
        let mut balance_flag = true;
        // println!("---------------------{}---------------", i);
        for c in case.chars() { 
            if opening.contains(&c) {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    println!("inner isempty {}", c);
                }
                let popped_open = stack.pop().unwrap();
                let expected_open = open_close_map.get(&c).unwrap();
                if *expected_open != popped_open {
                    balance_flag = false;
                }
            }
        }
        stack.reverse();
        if balance_flag {
            fixes.push(stack.into_iter().map(|x| close_open_map.get(&x).unwrap()).collect());
        }
    }
    // score it
    let mut scores: Vec<i64> = vec![];
    let mut total_score: i64 = 0;
    

    // println!("Fix strings: {:?}", fixes);
    for fix in fixes.iter() {
        total_score = 0;
        for c in fix.chars() {
            total_score *= 5;
            total_score += autocomplete_score_table.get(&c).unwrap();
        }
        scores.push(total_score);
    }

    scores.sort_unstable();

    println!("Scores: {:?}", scores);
    scores[(scores.len()/2)]
}
fn main() {
    let testfile = env::args().nth(1).unwrap();
    let input = fs::read_to_string(testfile).unwrap();
    let err_score_table: HashMap<char, i32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let opening = vec!['(', '[', '{', '<'];
    let open_close_map: HashMap<char, char> = HashMap::from([
        (')', '('),
        ('}', '{'),
        ('>', '<'),
        (']', '[')
    ]);
    let close_open_map: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('{', '}'),
        ('<', '>'),
        ('[', ']')
    ]);
    let autocomplete_score_table: HashMap<char, i64> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

    // println!("{}", part1(&input, err_score_table, open_close_map, opening));
    println!("{}", part2(&input, autocomplete_score_table, open_close_map, close_open_map, opening));


}
