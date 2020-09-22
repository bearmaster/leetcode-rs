impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let(mut left,mut right,mut ans) = (0 as usize,height.len() - 1,-1);
        while left < right {
            ans = ans.max(height[left].min(height[right])*(right as i32 - left as i32));
            if height[left] < height[right] { left += 1;}
            else { right -= 1; }
        }
        ans
    }
}

