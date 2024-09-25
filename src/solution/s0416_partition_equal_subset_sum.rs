#![allow(dead_code)]

/**
 * [416] Partition Equal Subset Sum
 *
 * Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,5,11,5]
 * Output: true
 * Explanation: The array can be partitioned as [1, 5, 5] and [11].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3,5]
 * Output: false
 * Explanation: The array cannot be partitioned into equal sum subsets.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 200
 * 	1 <= nums[i] <= 100
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/partition-equal-subset-sum/
// discuss: https://leetcode.com/problems/partition-equal-subset-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % 2 != 0 {
            return false;
        }
        let target = total_sum / 2;
        let mut dp = vec![false; target as usize + 1];
        dp[0] = true;

        for &num in nums.iter() {
            for i in (num as usize..=target as usize).rev() {
                dp[i] = dp[i] || dp[i - num as usize];
            }
        }

        dp[target as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_416 {
        use super::*;

        #[test]
        fn test_1() {
            let nums = vec![1, 5, 11, 5];
            let result = true;
            assert_eq!(Solution::can_partition(nums), result);
        }

        #[test]
        fn test_2() {
            let nums = vec![1, 2, 3, 5];
            let result = false;
            assert_eq!(Solution::can_partition(nums), result);
        }

        #[test]
        fn test_3() {
            let nums = vec![1, 2,  5];
            let result = false;
            assert_eq!(Solution::can_partition(nums), result);
        }
    }
}
