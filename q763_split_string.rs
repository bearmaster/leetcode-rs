impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        if s.len() == 1 {
		return vec![1];
	}
	let mut ans: Vec<i32> = vec![];
	// 26个英文字母最后出现的位置
	let mut v_right: Vec<usize> = vec![500; 26];
	let s: Vec<u8> = s.bytes().collect();
	let mut right = s.len()-1;
	//从右至左遍历，遇到第一个字符，退出循环，同时记录了字母出现的最后索引
	while right >= 0  {
		let ind = (s[right]-97) as usize;
		// 500表示未记录索引的字母
		if v_right[ind] == 500 {
			v_right[ind] = right;
		}
		// 如果不等以第一个字符，一直循环
		if s[right] != s[0] {
			right -= 1;
			continue;
		}
		if right == s.len()-1 {
			return vec![s.len() as i32];
		}
		let mut right_bound = right;
		let mut i = 1;
		// 计算从第一个字符开始最大的片段，然后退出循环
		while i <  right_bound {
			let ind = (s[i]-97) as  usize;
			if v_right[ind] < 500 {
				right_bound = right_bound.max(v_right[ind]);
			}
			i += 1;
		}
		right = right_bound + 1;
		ans.push(right as i32);
		break;
	}
	// 从上个循环的断点，继续向右循环
	while right < s.len() {
		let mut right_bound = v_right[(s[right]-97) as usize];
		let mut i = right + 1;
		while i < right_bound {
			right_bound = right_bound.max(v_right[(s[i]-97) as usize]) ;
			i += 1;
		} 
		ans.push((right_bound+1-right) as i32);
		right = right_bound+1;

	}

	return ans;
    }
}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];

        for (i, ch) in s.bytes().enumerate() {
            last[(ch - b'a') as usize] = i;
        }

        let mut l = 0;
        let mut r = 0;
        let mut ret = Vec::new();

        for (i, ch) in s.bytes().enumerate() {
            if i > r {
                ret.push((r - l) as i32 + 1);
                l = i;
                r = last[(ch - b'a') as usize];
            } else if last[(ch - b'a') as usize] > r {
                r = last[(ch - b'a') as usize];
            }
        }

        ret.push((r - l) as i32 + 1);

        ret
    }
}