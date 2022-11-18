use std::collections::HashMap;

pub struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map=HashMap::with_capacity(nums.len());
        for (index,num) in nums.iter().enumerate() {
            match map.get(&(target-num)) {
                Some(val)=>{
                    return  vec![*val as i32,index as i32];
                }
                None=>{
                    map.insert(*num,index);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){
        assert_eq!(vec![0,1],Solution::two_sum(vec![2,7,11,15],9));
        assert_eq!(vec![1,2],Solution::two_sum(vec![3,2,4],6));
    }
}