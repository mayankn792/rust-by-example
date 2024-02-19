// https://leetcode.com/problems/power-of-two/
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let bin = format!("{:b}", n);
        let mut found : bool = true;
        for ch in bin.chars().skip(1){
            if ch != '0' {
                found = false;
                break;
            }
        }
        
        return found;
    }
}