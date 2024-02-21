impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let sum : i32 = n * (n + 1) / 2;
        
        for i in 1 ..= n {
            let cur_sum : i32 = i * (i + 1) / 2;

            if sum - cur_sum + i == cur_sum {
                return i;
            }

        }
        return -1;
    }
}