impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let (mut L, mut R, mut sum) = (0, 0, 0);
        let mut best_res = 9999;
        let mut best_sum = 0;
        for i in 0..nums.len() {
            L = i + 1;
            R = nums.len() -1;
            while L < R {
                sum = nums[i] + nums[L] + nums[R];
                let res = sum - target;
                if res == 0 {
                    return sum;
                } else if res > 0 {
                    if res < best_res {
                        best_res = res;
                        best_sum = sum;
                    }
                    R -= 1;
                } else {
                    if -res < best_res {
                        best_res = -res;
                        best_sum = sum;
                    }
                    L += 1;
                }
            }
        }
        return best_sum;
    }
}