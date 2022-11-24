pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res:Vec<Vec<i32>>=vec![vec![0;n as usize];n as usize];
        let (mut l,mut r,mut t,mut b)=(0,n as usize -1 ,0,n as usize -1);
        let mut index=1;
        while index <= n * n {
            for i in l..=r {
                res[t][i]=index;
                index+=1;
            }
            t+=1;
            if index>n*n {
                break
            }
            for i in t..=b {
                res[i][r]=index;
                index+=1;
            }
            r-=1;
            if index>n*n {
                break
            }
            for i in (l..=r).rev() {
                res[b][i]=index;
                index+=1;
            }
            b-=1;
            if index>n*n {
                break
            }
            for i in (t..=b).rev() {
                res[i][l]=index;
                index+=1;
            }
            l+=1;
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use crate::solution::s0059::Solution;

    #[test]
    fn test_59(){
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5],]
        );
    }
}