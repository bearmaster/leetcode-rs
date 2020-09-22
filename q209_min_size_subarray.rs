
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if (nums.len() <= 0) {
            return 0;
        }
        if (nums[0] >= s) {
            return 1;
        }

        let mut l: usize = 0;
        let mut sum: i32 = 0;
        let mut min: i32 = std::i32::MAX;

        for (r, num) in nums.iter().enumerate() {
            sum += num;
            while sum >= s {
                // 收敛 最小连续数组长度
                let t = (r - l + 1) as i32;
                if t < min {
                    min = t;
                }

                sum -= nums[l]; // 左指针向右移动时，左右指针间距变小， 和变小
                l += 1;
            }
        }

        if min == std::i32::MAX {
            return 0;
        } else {
            return min;
        }
    }
}

class Solution {
    public:
    bool hasCycle(ListNode* head) 
    {
        //两个运动员位于同意起点head
        ListNode* faster{ head };  //快的运动员
        ListNode* slower{ head };  //慢的运动员
    
        if (head == NULL)  //输入链表为空，必然不是循环链表
            return false;
    
        while (faster != NULL && faster->next != NULL)
        {
            faster = faster->next->next;  //快的运动员每次跑两步
            slower = slower->next;  //慢的运动员每次跑一步
            if (faster == slower)  //他们在比赛中相遇了
                return true;  //可以断定是环形道，直道不可能相遇
        }
        return false;  //快的运动员到终点了，那就是直道，绕圈跑不会有终点
    }
    };
    
    作者：xi-yu-shi-liu-guang-3
    链接：https://leetcode-cn.com/problems/linked-list-cycle/solution/ckuai-man-zhi-zhen-dai-zhu-shi-by-xi-yu-shi-liu-gu/
    来源：力扣（LeetCode）
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。