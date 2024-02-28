use std::cmp::min;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut ans : Vec<String> = Vec::new();
        for ch in 'a' ..='z' {
            let mut status : bool = true;
            let mut count : i32 = 1000000;
            for word in words.iter() {
                if !word.contains(ch) {
                    status = false;
                    break;
                } else {
                    let mut cc : i32 = 0;
                    for c in word.chars() {
                        if c == ch {
                            cc += 1;
                        }
                    }

                    count = min(count, cc);
                }
            }

            if status {
                for _ in 0 .. count {
                    ans.push(ch.to_string());
                }
            }
        }

        return ans;
    }
}