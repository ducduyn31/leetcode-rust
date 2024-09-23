 #![allow(dead_code)]

/**
 * [509] Fibonacci Number
 *
 * The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,
 * 
 * F(0) = 0, F(1) = 1
 * F(n) = F(n - 1) + F(n - 2), for n > 1.
 * 
 * Given n, calculate F(n).
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 2
 * Output: 1
 * Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 3
 * Output: 2
 * Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 4
 * Output: 3
 * Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
 * 
 *  
 * Constraints:
 * 
 * 	0 <= n <= 30
 * 
 */


pub struct Solution {}

// problem: https://leetcode.com/problems/fibonacci-number/
// discuss: https://leetcode.com/problems/fibonacci-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut dp: [i32; 31] = [0; 31];
        dp[1] = 1;
        for i in 2..=n {
            dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
        }
        dp[n as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_509 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(Solution::fib(2), 1);
        }

        #[test]
        fn test_2() {
            assert_eq!(Solution::fib(3), 2);
        }

        #[test]
        fn test_3() {
            assert_eq!(Solution::fib(4), 3);
        }

        #[test]
        fn test_4() {
            assert_eq!(Solution::fib(30),  832040);
        }
    }
}