//https://leetcode.com/problems/missing-number/
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut r : i32 = nums.len() as i32;
        for (i, n) in nums.iter().enumerate() {
            r ^= i as i32;
            r ^= n;
        }

        return r;
    }
}