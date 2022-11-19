pub struct Solution{}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut  left=0;
        let mut  right=nums.len();
        while left<right {
            let mid=(left+right)/2;
            if nums[mid]<target {
                left=mid+1;
            }else if nums[mid]>target {
                right=mid;
            }else {
                return mid as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_704(){
        assert_eq!(4,Solution::search(vec![-1,0,3,5,9,12],9));
        assert_eq!(-1,Solution::search(vec![-1,0,3,5,9,12],2));
    }
}