// don't mind this solution ... I just wanted to get the code to compile
// and I'm still unclear on &str, String, and how to get iterators going


use bimap::BiMap;
use itertools::Itertools;

fn count_seq(a: &str, b: &str) -> i32 {
    // returns how many elem in a appear in b
    let mut count = 0;
    for i in a.chars() {
        if b.contains(i) {
            count += 1
        }
    }
    count
}



pub trait RemoveMultiple<T> {
    /// Remove multiple indices
    fn remove_multiple(&mut self, to_remove: Vec<usize>);
    
    /// Remove multiple indices with swap_remove, this is faster but reorders elements
    fn swap_remove_multiple(&mut self, to_remove: Vec<usize>);
    
    /// Remove and return multiple indices
    fn take_multiple(&mut self, to_remove: Vec<usize>) -> Vec<T>;
        
    /// Remove and return multiple indices, preserving the order specified in the index list
    fn take_multiple_in_order(&mut self, to_remove: &[usize]) -> Vec<T>;
    
    /// Remove and return multiple indices with swap_remove, this is faster but reorders elements and the results are in reverse order
    fn swap_take_multiple(&mut self, to_remove: Vec<usize>) -> Vec<T>;
}

impl<T> RemoveMultiple<T> for Vec<T> {
    // https://stackoverflow.com/questions/57947441/remove-a-sequence-of-values-from-a-vec-in-rust
    fn remove_multiple(&mut self, mut to_remove: Vec<usize>) {
        to_remove.sort();
        to_remove.reverse();
        for r in to_remove {
            self.remove(r);
        }
    }
    
    fn swap_remove_multiple(&mut self, mut to_remove: Vec<usize>) {
        to_remove.sort();
        to_remove.reverse();
        for r in to_remove {
            self.swap_remove(r);
        }
    }
    
    fn take_multiple(&mut self, mut to_remove: Vec<usize>) -> Vec<T> {
        to_remove.sort();
        to_remove.reverse();
        let mut collected = vec![];
        for r in to_remove {
            collected.push(self.remove(r));
        }
        collected.reverse();
        collected
    }
    
    fn take_multiple_in_order(&mut self, to_remove: &[usize]) -> Vec<T> {
        let mut to_remove = to_remove.iter().copied().enumerate().collect::<Vec<_>>();
        to_remove.sort_by_key(|(_, r)| *r);
        to_remove.reverse();
        let mut collected : Vec<Option<T>> = std::iter::repeat_with(|| None).take(to_remove.len()).collect();
        for (i, r) in to_remove {
            collected[i] = Some(self.remove(r));
        }
        collected.into_iter().filter_map(|x| x).collect()
    }
    
    fn swap_take_multiple(&mut self, mut to_remove: Vec<usize>) -> Vec<T> {
        to_remove.sort();
        to_remove.reverse();
        let mut collected = vec![];
        for r in to_remove {
            collected.push(self.swap_remove(r));
        }
        collected
    }
}






fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split('\n')
        .collect::<Vec<&str>>();

    // part 1a
    // In the output values, how many times do digits 1, 4, 7, or 8 appear?
    // 0: a b c   e f g : 6
    // 1:     c     f   : 2
    // 2: a   c d e   g : 5
    // 3: a   c d   f g : 5
    // 4:   b c d   f   : 4
    // 5: a b   d   f g : 5
    // 6: a b   d e f g : 6
    // 7: a   c     f   : 3
    // 8: a b c d e f g : 7
    // 9: a b c d   f g : 6

    // part2: decode!
    // do the following in order
    // 1) 1, 4, 7, 8 are known
    // 2) 3 is the only 5-segment which contains the segments of 1
    // 3) 9 is the only 6-segment which contains the segments of 3
    // 4) 0 is the only remaining 6-segment which contains the segments of 1
    // 5) 6 must be the remaining 6-segment
    // 6) 2 is the 5-segment which contains 2 segments of 4
    // 7) 5 is the 5-segment which contains 3 segments of 4

    let mut total = 0;

    // part b
    for line in input.iter() {
        let mut seq_map: BiMap<String, i32> = BiMap::new();
        let cur_sequence = line
            .split('|')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let seq_notes_raw = cur_sequence[0].split_whitespace().collect::<Vec<&str>>();
        // I don't know the requisite iterator-foo to do this in-place
        let mut seq_notes = vec![];

        for i in seq_notes_raw {
            let t = i.chars().sorted().collect::<String>();
            seq_notes.push(t);
        }









        // build map

        // base cases based off of the counts
        let mut pos = seq_notes.iter().position(|x| x.len() == 2).unwrap();
        seq_map.insert(seq_notes[pos].clone(), 1);
        seq_notes.remove(pos);
        pos = seq_notes.iter().position(|x| x.len() == 3).unwrap();
        seq_map.insert(seq_notes[pos].clone(), 7);
        seq_notes.remove(pos);
        pos = seq_notes.iter().position(|x| x.len() == 4).unwrap();
        seq_map.insert(seq_notes[pos].clone(), 4);
        seq_notes.remove(pos);
        pos = seq_notes.iter().position(|x| x.len() == 7).unwrap();
        seq_map.insert(seq_notes[pos].clone(), 8);
        seq_notes.remove(pos);
        // 2) 3 is the only 5-segment which contains the segments of 1
        // 6) 2 is the only 5-segment which contains 2 segments of 4
        // 7) 5 is the only 5-segment which contains 3 segments of 4
        let mut removeable: Vec<usize> = vec![];
        for (i, seq) in seq_notes.iter().filter(|x| x.len() == 5).enumerate() {
            if count_seq(seq_map.get_by_right(&1).unwrap(), seq) == 2 {
                seq_map.insert(seq.to_string(), 3);
                pos = seq_notes.iter().position(|x| x == seq).unwrap();
                removeable.push(pos);
            } else if count_seq(seq_map.get_by_right(&4).unwrap(), seq) == 2 {
                seq_map.insert(seq.to_string(), 2);
                pos = seq_notes.iter().position(|x| x == seq).unwrap();
                removeable.push(pos);
            } else if (count_seq(seq_map.get_by_right(&4).unwrap(), seq) == 3)
                & (count_seq(seq_map.get_by_right(&1).unwrap(), seq) == 1)
            {
                seq_map.insert(seq.to_string(), 5);
                pos = seq_notes.iter().position(|x| x == seq).unwrap();
                removeable.push(pos);
            }
        }
        seq_notes.remove_multiple(removeable);
        removeable = vec![];


        // 3) 9 is the only 6-segment which contains the all segments of 3
        // TODO: This isn't removing properly!
        for (i, seq) in seq_notes.iter().filter(|x| x.len() == 6).enumerate() {
            if count_seq(seq_map.get_by_right(&3).unwrap(), seq) == 5 {
                seq_map.insert(seq.to_string(), 9);
                pos = seq_notes.iter().position(|x| x == seq).unwrap();
                removeable.push(pos);
                break;
            }
        }
        assert!(seq_map.len() == 8);


        seq_notes.remove_multiple(removeable);
        removeable= vec![];
        // 4) 0 is the only remaining 6-segment which contains the segments of 1
        for (i, seq) in seq_notes.iter().filter(|x| x.len() == 6).enumerate() {

            if count_seq(seq_map.get_by_right(&1).unwrap(), seq) == 2 {
                seq_map.insert(seq.to_string(), 0);
                pos = seq_notes.iter().position(|x| x == seq).unwrap();
                removeable.push(pos);
            }
        }

        seq_notes.remove_multiple(removeable);
        seq_map.insert(seq_notes[0].clone(), 6);


        let mut decode_msg_raw = cur_sequence[1].split_whitespace().collect::<Vec<&str>>();
        let mut decoded_value = 0;


        let mut decode_msg = vec![];

        for i in decode_msg_raw {
            let t = i.chars().sorted().collect::<String>();
            decode_msg.push(t);
        }
        println!("{:?}", decode_msg);
        println!("{:?}", seq_map);

        for (i, num) in decode_msg.iter().enumerate() {
            decoded_value += (i32::pow(10, 3-i as u32)) * seq_map.get_by_left(num).unwrap();
        }
        println!("decoded_value: {}", decoded_value);

        total += decoded_value;


    }


    println! {"total: {}", total};
}
