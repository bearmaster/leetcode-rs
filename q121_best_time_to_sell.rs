use std::cmp::{min, max};
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        let mut minum = prices[0];
        let mut maxium = 0;
        for v in prices {
            maxium = maxium.max(v - minum);
            minum = minum.min(v);
        }
        maxium
    }
}
