//https://leetcode.com/problems/kth-missing-positive-number/
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut count = 1;
        let mut ptr = 0;
        let mut miss = 0;
        loop {
            let size = arr.len();
            if let Some(ele) = arr.get(ptr) {
                if count != *ele {
                    miss += 1;
                } else {
                    ptr += 1
                }
                count += 1;
            } else {
                count += 1;
                miss += 1;
            }

            if miss == k {
                break;
            }
        }

        return count - 1;
    }
}