//https://leetcode.com/problems/first-unique-character-in-a-string/?envType=daily-question&envId=2024-02-05
use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_counts: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        for (i, c) in s.chars().enumerate() {
            let cnt_opt = char_counts.get(&c);
            match cnt_opt {
                Some(value) => {
                    if *value == 1 {
                        return i as i32
                    }
                },
                None => {
                    println!("Value is NULL.")
                },
            }
        }

        return -1
    }
}