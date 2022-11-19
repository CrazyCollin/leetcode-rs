pub struct Solution{}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut start=0;
        let mut res=i32::MAX;
        let mut sum=0;
        for i in 0..nums.len() {
            sum+=nums[i];
            while sum >= target {
                // 缩小窗口
                let sub_length=(i-start+1) as i32;
                if sub_length<res {
                    res=sub_length;
                }
                sum-=nums[start];
                start+=1;
            }
        }
        if res==i32::MAX {
            return 0;
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use crate::solution::s0209::Solution;

    #[test]
    fn test_209(){
        assert_eq!(2,Solution::min_sub_array_len(7,vec![2,3,1,2,4,3]));
    }
}