use std::iter::{self, Sum};

#[derive(Debug)]
struct BingoCardEntry { 
    number: u32,
    is_called: bool,
}


#[derive(Debug)]
struct BingoCard { 
    entries: Vec<BingoCardEntry>,
    score: u32,
    has_won: bool,
    // unwrap like this:
    // [[00, 01, 02, 03, 04]
    // [05, 06, 07, 08, 09]
    // ...]
    // so x, y -> [y*5 + x]
}

impl BingoCard{
    pub fn is_win(& mut self) -> bool {
        let mut win = false;

        // check rows
        for i in 0..5 {
            if self.entries[i*5 .. i*5 + 5].iter().all(|e| e.is_called) {
                win = true;
            }
        }


        // check columns
        for i in 0..5 {
            if self.entries.iter().skip(i).step_by(5).all(|e| e.is_called) {
                win = true;
            }
        }
        // diagonals don't count
        self.has_won = win;

        win
    }

    pub fn display_called(&self) {
        println!("{:?}", self.entries.iter().filter(|e| e.is_called).map(|e| e.number).collect::<Vec<u32>>());
    }

    pub fn score_card(&mut self, just_called: u32) {
        let unmarked_cards = self.entries.iter()
            .filter(|x| !x.is_called);
        let mut sum = 0;
        for i in unmarked_cards {
            sum += i.number;
        }
        self.score = sum * just_called
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        self.entries[x + y * 5].number
    }

    pub fn set(&mut self, x: usize, y: usize, number: u32) {
        self.entries[x + y * 5].number = number;
    }

    pub fn call(&mut self, n: u32) {

        for entry in self.entries.iter_mut() {
            if entry.number == n {
                entry.is_called = true;
            }
        }

    }
}


fn build_cards(input: Vec<&str>) -> Result<Vec<BingoCard>, String> {
    if input.is_empty() {
        return Err("no input".to_string());
    }
    let mut card_descriptors = input.iter();
    let _ = card_descriptors.next(); // skip first line
    let mut cards: Vec<BingoCard> = vec![];

    while let Some(line) = card_descriptors.next() {
        if line.is_empty() {
            continue;
        }

        let mut card_entries: Vec<u32> = vec![];
        
        card_entries.extend(line.split_whitespace().filter_map(|w| w.parse().ok()).collect::<Vec<u32>>());
        for _ in 0..5{
            card_entries.extend(card_descriptors.next().unwrap()
                .split_whitespace().filter_map(|w| w.parse().ok()).collect::<Vec<u32>>());
        }

        if card_entries.len() != 25 {
            return Err(format!("card has {} entries, expected 25", card_entries.len()));
        }

        cards.push(BingoCard {
            entries: card_entries.iter().map(
                         |n| BingoCardEntry{number: *n, is_called: false})
                .collect(),
            score: 0, has_won: false
        });
    }

    Ok(cards)
}

fn main() {
    let input = include_str!("../input.txt").split('\n').collect::<Vec<&str>>();
    
    let mut input_iter = input.iter();

    let nums = input_iter.next().unwrap().split(',').filter_map(|w| w.parse().ok()).collect::<Vec<u32>>();

    let mut cards = build_cards(input.clone()).unwrap();

    // for part 1 -- finding score of 1st winner
    /* for num in nums {
        for card in & mut cards {
            (*card).call(num);
            if (*card).is_win() {
                (*card).score_card(num);
                println!("score: {}", (*card).score);
                return; // we only want the first one
            }
        }
    } */

    // for part 2 -- finding score of last winner
    let mut last_score = 0;
    for num in nums {
        for card in & mut cards {
            if (*card).has_won {continue;}
            (*card).call(num);
            if (*card).is_win() {
                (*card).score_card(num);
                last_score = (*card).score;
            }
        }
    }
    println!("score: {}", last_score);




    



}




















