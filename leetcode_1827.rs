//https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let size = nums.len();
        for i in 1 .. size {
            let prev = nums[i - 1];
            let curr = nums[i];

            if prev >= curr {
                let inc = prev - curr + 1;
                res += inc;
                nums[i] += inc;
            }
        }

        return res;
    }
}