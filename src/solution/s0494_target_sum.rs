#![allow(dead_code)]

/**
 * [494] Target Sum
 *
 * You are given an integer array nums and an integer target.
 * You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.
 *
 * 	For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them to build the expression "+2-1".
 *
 * Return the number of different expressions that you can build, which evaluates to target.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,1,1,1,1], target = 3
 * Output: 5
 * Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
 * -1 + 1 + 1 + 1 + 1 = 3
 * +1 - 1 + 1 + 1 + 1 = 3
 * +1 + 1 - 1 + 1 + 1 = 3
 * +1 + 1 + 1 - 1 + 1 = 3
 * +1 + 1 + 1 + 1 - 1 = 3
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1], target = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 20
 * 	0 <= nums[i] <= 1000
 * 	0 <= sum(nums[i]) <= 1000
 * 	-1000 <= target <= 1000
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/target-sum/
// discuss: https://leetcode.com/problems/target-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // A simulation of the problem
        // the row header is how many numbers from the beginning of the array we have considered
        // the column header is the sum of the numbers we have considered
        // the value is the number of ways to get the sum
        // target = 3, nums = [1, 1, 1, 1, 1]
        // sum: -5          -4          -3          -2          -1          0          1          2          3          4          5
        // 0:        0            0           0           0           0          1          0          0          0          0          0
        // 1:        0            0           0           0           1          0          1          0          0          0          0
        // 2:        0            0           0           1           0          2          0          1          0          0          0
        // 3:        0            0           1           0           3          0          3          0          1          0          0
        // 4:        0            1           0           4           0          6          0          4          0          1          0
        // 5:        1            0           5           0           10         0          10         0          5          0          1
        //
        let total_sum = nums.iter().sum::<i32>() as usize;

        if target.abs() > total_sum as i32 || (total_sum as i32 + target) % 2 != 0 {
            return 0;
        }

        // We can create a 2D array to store as the table above
        // But we can optimize the space complexity by using a 1D array since we only need the previous row
        let p = (total_sum + target as usize) / 2;
        let mut dp = vec![0; p + 1];

        // Initialize the first row
        dp[0] = 1;

        for i in 0..nums.len() {
            for j in (nums[i] as usize..=p).rev() {
                dp[j] += dp[j - nums[i] as usize];
            }
        }

        dp[p]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_494 {
        use super::*;

        #[test]
        fn test_1() {
            let nums = vec![1, 1, 1, 1, 1];
            let target = 3;
            let result = 5;
            assert_eq!(Solution::find_target_sum_ways(nums, target), result);
        }

        #[test]
        fn test_2() {
            let nums = vec![1];
            let target = 1;
            let result = 1;
            assert_eq!(Solution::find_target_sum_ways(nums, target), result);
        }

        #[test]
        fn test_3() {
            let nums = vec![1];
            let target = 2;
            let result = 0;
            assert_eq!(Solution::find_target_sum_ways(nums, target), result);
        }

        #[test]
        fn test_4() {
            let nums = vec![7, 9, 3, 8, 0, 2, 4, 8, 3, 9];
            let target = 0;
            let result = 0;
            assert_eq!(Solution::find_target_sum_ways(nums, target), result);
        }
    }
}
