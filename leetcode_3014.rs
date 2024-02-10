//https://leetcode.com/problems/number-of-lines-to-write-string/
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut frame : i32  = 0;
        let mut cnt : i32 = 0;
        
        for ch in s.chars() {
            let idx = (ch as u8 - 'a' as u8) as usize;
            let w : i32 = widths[idx];

            if frame + w > 100 {
                cnt += 1;
                frame = w;
            } else {
                frame += w;
            }
        }

        let mut ans : Vec<i32> = Vec::new();
        if frame > 0 {
            cnt += 1
        }
        ans.push(cnt);
        ans.push(frame);

        return ans
    }
}