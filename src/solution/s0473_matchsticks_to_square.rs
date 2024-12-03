#![allow(dead_code)]

/**
 * [473] Matchsticks to Square
 *
 * You are given an integer array matchsticks where matchsticks[i] is the length of the i^th matchstick. You want to use all the matchsticks to make one square. You should not break any stick, but you can link them up, and each matchstick must be used exactly one time.
 * Return true if you can make this square and false otherwise.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/matchsticks1-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: matchsticks = [1,1,2,2,2]
 * Output: true
 * Explanation: You can form a square with length 2, one side of the square came two sticks with length 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: matchsticks = [3,3,3,3,4]
 * Output: false
 * Explanation: You cannot find a way to form a square with all the matchsticks.
 *
 *  
 * Constraints:
 *
 * 	1 <= matchsticks.length <= 15
 * 	1 <= matchsticks[i] <= 10^8
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/matchsticks-to-square/
// discuss: https://leetcode.com/problems/matchsticks-to-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let perimeter: i32 = matchsticks.iter().sum();

        if perimeter % 4 > 0 {
            return false;
        }
        let side = perimeter / 4;
        let mut matchsticks = matchsticks;
        matchsticks.sort();
        matchsticks.reverse();

        if matchsticks[0] > side {
            return false;
        }

        let mut sides = vec![side; 4];
        Self::dfs(&matchsticks, &mut sides, 0, side)
    }

    fn dfs(nums: &Vec<i32>, sides: &mut Vec<i32>, index: usize, side: i32) -> bool {
        if index == nums.len() {
            return sides[0] == sides[1] && sides[1] == sides[2] && sides[2] == sides[3];
        }

        for i in 0..4 {
            if sides[i] - nums[index] < 0 {
                continue;
            }

            sides[i] -= nums[index];
            if Self::dfs(nums, sides, index + 1, side) {
                return true;
            }
            sides[i] += nums[index];
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_473 {
        use super::*;

        #[test]
        fn test_1() {
            let matchsticks = vec![1, 1, 2, 2, 2];
            let result = true;

            assert_eq!(Solution::makesquare(matchsticks), result);
        }

        #[test]
        fn test_2() {
            let matchsticks = vec![3, 3, 3, 3, 4];
            let result = false;

            assert_eq!(Solution::makesquare(matchsticks), result);
        }
    }
}

