pub struct Solution{}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut start,mut end)=(0,nums.len()-1);
        // let mut res=Vec::with_capacity(nums.len());
        let mut res=vec![];
        while start <= end {
            let (left_max, right_max)=(nums[start]*nums[start], nums[end]*nums[end]);
            if left_max>right_max {
                res.push(left_max);
                start+=1;
            }else {
                res.push(right_max);
                end-=1;
            }
        }
        res.reverse();
        res
    }

}

#[cfg(test)]
mod tests{
    use crate::solution::s0977::Solution;

    fn equal(a:Vec<i32>,b:Vec<i32>)->bool{
        for (index,num) in a.iter().enumerate() {
            if *num!=b[index] {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_977(){
        assert!(equal(vec![0,1,9,16,100],Solution::sorted_squares(vec![-4,-1,0,3,10])));
    }
}