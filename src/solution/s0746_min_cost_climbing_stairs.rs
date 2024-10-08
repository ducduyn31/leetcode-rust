#![allow(dead_code)]

/**
 * [746] Min Cost Climbing Stairs
 *
 * You are given an integer array cost where cost[i] is the cost of i^th step on a staircase. Once you pay the cost, you can either climb one or two steps.
 * You can either start from the step with index 0, or the step with index 1.
 * Return the minimum cost to reach the top of the floor.
 *  
 * <strong class="example">Example 1:
 *
 * Input: cost = [10,<u>15</u>,20]
 * Output: 15
 * Explanation: You will start at index 1.
 * - Pay 15 and climb two steps to reach the top.
 * The total cost is 15.
 *
 * <strong class="example">Example 2:
 *
 * Input: cost = [<u>1</u>,100,<u>1</u>,1,<u>1</u>,100,<u>1</u>,<u>1</u>,100,<u>1</u>]
 * Output: 6
 * Explanation: You will start at index 0.
 * - Pay 1 and climb two steps to reach index 2.
 * - Pay 1 and climb two steps to reach index 4.
 * - Pay 1 and climb two steps to reach index 6.
 * - Pay 1 and climb one step to reach index 7.
 * - Pay 1 and climb two steps to reach index 9.
 * - Pay 1 and climb one step to reach the top.
 * The total cost is 6.
 *
 *  
 * Constraints:
 *
 * 	2 <= cost.length <= 1000
 * 	0 <= cost[i] <= 999
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/min-cost-climbing-stairs/
// discuss: https://leetcode.com/problems/min-cost-climbing-stairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len()];
        dp[0] = cost[0];
        dp[1] = cost[1];
        for i in 2..cost.len() {
            dp[i] = cost[i] + dp[i - 1].min(dp[i - 2]);
        }

        dp[cost.len() - 1].min(dp[cost.len() - 2])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_746 {
        use super::*;

        #[test]
        fn test_1() {
            let cost = vec![10, 15, 20];
            let result = 15;
            assert_eq!(Solution::min_cost_climbing_stairs(cost), result);
        }

        #[test]
        fn test_2() {
            let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
            let result = 6;
            assert_eq!(Solution::min_cost_climbing_stairs(cost), result);
        }
    }
}
