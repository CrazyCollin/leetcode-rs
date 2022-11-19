pub struct Solution{}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut res=0;
        for i in 0..nums.len() {
            if nums[i]!=val {
                nums[res]=nums[i];
                res+=1;
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests{
    use crate::solution::s0027::Solution;

    #[test]
    fn test_27(){
        let mut v=vec![3,2,2,3];
        assert_eq!(2,Solution::remove_element(&mut v,3));
    }
}