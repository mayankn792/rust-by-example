//https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/
impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut min: i32 = 0;
        for word in s.split_whitespace() {
            match word.parse::<i32>() {
                Ok(num) => {
                    if min >= num {
                        return false;
                    }
                    min = num;
                } Err(err) => {}
            }
        }

        return true;
    }
}