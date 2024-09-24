#![allow(dead_code)]

/**
 * [918] Maximum Sum Circular Subarray
 *
 * Given a circular integer array nums of length n, return the maximum possible sum of a non-empty subarray of nums.
 * A circular array means the end of the array connects to the beginning of the array. Formally, the next element of nums[i] is nums[(i + 1) % n] and the previous element of nums[i] is nums[(i - 1 + n) % n].
 * A subarray may only include each element of the fixed buffer nums at most once. Formally, for a subarray nums[i], nums[i + 1], ..., nums[j], there does not exist i <= k1, k2 <= j with k1 % n == k2 % n.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,-2,3,-2]
 * Output: 3
 * Explanation: Subarray [3] has maximum sum 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [5,-3,5]
 * Output: 10
 * Explanation: Subarray [5,5] has maximum sum 5 + 5 = 10.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [-3,-2,-3]
 * Output: -2
 * Explanation: Subarray [-2] has maximum sum -2.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 3 * 10^4
 * 	-3 * 10^4 <= nums[i] <= 3 * 10^4
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-circular-subarray/
// discuss: https://leetcode.com/problems/maximum-sum-circular-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max_current = nums[0];
        let mut max_global = nums[0];
        let mut min_current = nums[0];
        let mut min_global = nums[0];
        let mut total = nums[0];

        for i in 1..nums.len() {
            total += nums[i];
            max_current = nums[i].max(max_current + nums[i]);
            max_global = max_global.max(max_current);

            min_current = nums[i].min(min_current + nums[i]);
            min_global = min_global.min(min_current);
        }

        if min_global == total {
            max_global
        } else {
            max_global.max(total - min_global)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_918 {
        use super::*;

        #[test]
        fn test_1() {
            let nums = vec![1, -2, 3, -2];
            let result = 3;
            assert_eq!(Solution::max_subarray_sum_circular(nums), result);
        }

        #[test]
        fn test_2() {
            let nums = vec![5, -3, 5];
            let result = 10;
            assert_eq!(Solution::max_subarray_sum_circular(nums), result);
        }

        #[test]
        fn test_3() {
            let nums = vec![-3, -2, -3];
            let result = -2;
            assert_eq!(Solution::max_subarray_sum_circular(nums), result);
        }
    }
}
