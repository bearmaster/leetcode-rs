fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // 不够三个数, 直接返回空数组
    if nums.len() < 3 {
        return vec![];
    }
    
    // 找不到头绪, 先排个序吧
    let mut nums = nums.clone();
    nums.sort();
    
    let mut result = vec![];
    
    // 遍历, 先选定三数中的第一个数的坐标 a
    let mut a = 0;
    loop {
        // a 最多只能是整个数组里的倒数第三个数
        if a == nums.len()-2 || !(nums[a] <= 0) {
            break;
        }
        if a > 0 && nums[a] == nums[a-1] {
            //重复的数字， 继续
            a += 1;
            continue;
        }
        // 双指针对撞， 回到两数之和的问题
        let mut b = a + 1;
        let mut c = nums.len()-1;
        loop {
            if b >= c {
                // 相撞，找不到能和a组合的b，c， 结束循环
                break;
            }
            let sum = nums[a] + nums[b] + nums[c];
            if sum == 0 {
                // 找到了, 加到结果里
                result.push(vec![nums[a], nums[b], nums[c]]);
                // 跳过重复的结果
                while b < c && nums[b] == nums[b+1] {
                    b += 1;
                }
                while b < c && nums[c] == nums[c-1] {
                    c -= 1;
                } 
                // 寻找别的组合
                b += 1;
                c -= 1;
                continue;
            } else if sum < 0 {
                // 太小了, b 👉 移动
                b += 1;
                continue;
            } else {
                // 太大， c 👈 移动
                c -= 1;
                continue;
            }
        }
        a += 1;
    }
    result
}

