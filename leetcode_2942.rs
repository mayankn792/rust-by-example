//https://leetcode.com/problems/find-words-containing-character/
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut ind: Vec<i32> = Vec::new();

        for (i, word) in words.iter().enumerate() {
            let mut flag: bool = false;
            for ch in word.chars() {
                if ch == x {
                    ind.push(i as i32);
                    flag = true;
                    break;
                }
            }
        }

        return ind;
    }
}