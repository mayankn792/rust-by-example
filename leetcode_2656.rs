//https://leetcode.com/problems/maximum-sum-with-exactly-k-elements/description/
impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = nums[0];
        for num in nums {
            if num > max {
                max = num;
            }
        }

        let mut ans: i32 = 0;
        for _ in 0 .. k {
            ans += max;
            max += 1;
        }

        return ans;
    }
}