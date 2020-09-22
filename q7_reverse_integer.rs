use std::i32;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let max = (i32::MAX - 7) / 10;
        let min = (i32::MIN + 8) / 10;

        let mut x = x;
        let mut r = 0;

        while x != 0 {
            let m = x % 10;
            if r > max || (r == max && m > 7) || r < min || (r == min && m < -8) {
                return 0;
            }

            r = r * 10 + m;
            x = (x - m) / 10;
        }

        r
    }
}