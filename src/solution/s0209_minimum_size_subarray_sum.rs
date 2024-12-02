#![allow(dead_code)]

/**
 * [209] Minimum Size Subarray Sum
 *
 * Given an array of positive integers nums and a positive integer target, return the minimal length of a <span data-keyword="subarray-nonempty">subarray</span> whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.
 *  
 * <strong class="example">Example 1:
 *
 * Input: target = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: The subarray [4,3] has the minimal length under the problem constraint.
 *
 * <strong class="example">Example 2:
 *
 * Input: target = 4, nums = [1,4,4]
 * Output: 1
 *
 * <strong class="example">Example 3:
 *
 * Input: target = 11, nums = [1,1,1,1,1,1,1,1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= target <= 10^9
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^4
 *
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-size-subarray-sum/
// discuss: https://leetcode.com/problems/minimum-size-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut current_sum = 0;
        let mut result = i32::MAX;

        for right in 0..nums.len() {
            current_sum += nums[right];

            while current_sum >= target {
                result = result.min((right - left + 1) as i32);
                current_sum -= nums[left];
                left += 1;
            }
        }

        if result == i32::MAX {
            0
        } else {
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_209 {
        use super::*;

        #[test]
        fn test_1() {
            let target = 7;
            let nums = vec![2, 3, 1, 2, 4, 3];
            let result = 2;
            assert_eq!(Solution::min_sub_array_len(target, nums), result)
        }

        #[test]
        fn test_2() {
            let target = 4;
            let nums = vec![1, 4, 4];
            let result = 1;
            assert_eq!(Solution::min_sub_array_len(target, nums), result)
        }

        #[test]
        fn test_3() {
            let target = 11;
            let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
            let result = 0;
            assert_eq!(Solution::min_sub_array_len(target, nums), result)
        }
    }
}
