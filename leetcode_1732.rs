//https://leetcode.com/problems/find-the-highest-altitude
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max : i32 = 0;
        let mut count : i32 = 0;
        for g in gain.iter() {
            count += g;
            max = i32::max(max, count)
        }

        return max;
    }
}