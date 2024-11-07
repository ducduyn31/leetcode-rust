#![allow(dead_code)]

use std::collections::VecDeque;

/**
 * [279] Perfect Squares
 *
 * Given an integer n, return the least number of perfect square numbers that sum to n.
 * A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 12
 * Output: 3
 * Explanation: 12 = 4 + 4 + 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 13
 * Output: 2
 * Explanation: 13 = 4 + 9.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-squares/
// discuss: https://leetcode.com/problems/perfect-squares/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![n; (n + 1) as usize];
        let mut perfect_squares: Vec<i32> = Vec::new();
        let mut queue: VecDeque<i32> = VecDeque::new();

        for i in 1..n {
            if i * i <= n {
                perfect_squares.push(i * i);
                queue.push_back(i * i);
                dp[(i * i) as usize] = 1;
            } else {
                break;
            }
        }

        if dp[n as usize] == 1 {
            return 1;
        }

        while let Some(item) = queue.pop_front() {
            for k in &perfect_squares {
                if item + k == n {
                    return dp[item as usize] + 1;
                } else if item + k < n && dp[(item + k) as usize] > dp[item as usize] + 1 {
                    dp[(item + k) as usize] = dp[item as usize] + 1;
                    queue.push_back(item + k);
                }
            }
        }

        dp[n as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_279 {
        use super::*;

        #[test]
        fn test_1() {
            let n = 12;
            let result = 3;
            assert_eq!(Solution::num_squares(n), result);
        }

        #[test]
        fn test_2() {
            let n = 13;
            let result = 2;
            assert_eq!(Solution::num_squares(n), result);
        }

        #[test]
        fn test_3() {
            let n = 4;
            let result = 1;
            assert_eq!(Solution::num_squares(n), result);
        }
    }
}

