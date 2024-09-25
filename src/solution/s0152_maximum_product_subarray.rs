#![allow(dead_code)]

/**
 * [152] Maximum Product Subarray
 *
 * Given an integer array nums, find a <span data-keyword="subarray-nonempty">subarray</span> that has the largest product, and return the product.
 * The test cases are generated so that the answer will fit in a 32-bit integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,3,-2,4]
 * Output: 6
 * Explanation: [2,3] has the largest product 6.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [-2,0,-1]
 * Output: 0
 * Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	-10 <= nums[i] <= 10
 * 	The product of any subarray of nums is guaranteed to fit in a 32-bit integer.
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-subarray/
// discuss: https://leetcode.com/problems/maximum-product-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut global_max = nums[0];
        let mut local_max = nums[0];
        // We want to keep track of the local min as well, because if local min is negative, and the next
        // number is negative, then the product of the two will be positive, which can be the maximum product
        let mut local_min = nums[0];

        for i in 1..nums.len()  {
            let last_local_max = local_max;
            // There are three possibilities that the window can be set:
            // 1. The window is the current number, when the previous window is negative, and there isn't enough
            //    numbers to make the product positive, so we start a new window Eg: [-2, 3, -4, 5, -1, 0]
            // 2. The window can be extended by the current number, (with the first case) Eg: [-2, 3, 4, 5, -1, 0]
            // 3. The current number will flip the local min window to positive, creating a new window
            // starting from the local min window index Eg: [2, 3, -2, 4, 5, -1, 0]
            local_max = nums[i].max(nums[i] * local_max).max(nums[i] * local_min);

            // We mainly care about the magnitude of the product, so with higher magnitude, we can get a higher product
            // So we can just keep track of the minimum product, and multiply it with the current number (if it is negative)
            // to get the maximum
            // The window here is the same as the local max window, but we are looking for the minimum product
            local_min = nums[i].min(nums[i] * last_local_max).min(nums[i] * local_min);
            global_max = global_max.max(local_max);
        }

        global_max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_152 {
        use super::*;

        #[test]
        fn test_1() {
            let nums = vec![2, 3, -2, 4];
            let result = 6;
            assert_eq!(Solution::max_product(nums), result);
        }

        #[test]
        fn test_2() {
            let nums = vec![-2, 0, -1];
            let result = 0;
            assert_eq!(Solution::max_product(nums), result);
        }

        #[test]
        fn test_3() {
            let nums = vec![2, 3, -2, 4, 5, -1, 0];
            let result = 240;
            assert_eq!(Solution::max_product(nums), result);
        }
    }
}
