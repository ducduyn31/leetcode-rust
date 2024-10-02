#![allow(dead_code)]

/**
 * [1049] Last Stone Weight II
 *
 * You are given an array of integers stones where stones[i] is the weight of the i^th stone.
 * We are playing a game with the stones. On each turn, we choose any two stones and smash them together. Suppose the stones have weights x and y with x <= y. The result of this smash is:
 *
 * 	If x == y, both stones are destroyed, and
 * 	If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
 *
 * At the end of the game, there is at most one stone left.
 * Return the smallest possible weight of the left stone. If there are no stones left, return 0.
 *  
 * <strong class="example">Example 1:
 *
 * Input: stones = [2,7,4,1,8,1]
 * Output: 1
 * Explanation:
 * We can combine 2 and 4 to get 2, so the array converts to [2,7,1,8,1] then,
 * we can combine 7 and 8 to get 1, so the array converts to [2,1,1,1] then,
 * we can combine 2 and 1 to get 1, so the array converts to [1,1,1] then,
 * we can combine 1 and 1 to get 0, so the array converts to [1], then that's the optimal value.
 *
 * <strong class="example">Example 2:
 *
 * Input: stones = [31,26,33,21,40]
 * Output: 5
 *
 *  
 * Constraints:
 *
 * 	1 <= stones.length <= 30
 * 	1 <= stones[i] <= 100
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/last-stone-weight-ii/
// discuss: https://leetcode.com/problems/last-stone-weight-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        // stones: [2, 7, 4, 1, 8, 1]
        // sum: 23
        let mut sum = 0usize;
        let mut max = 0usize;

        for i in 0..stones.len() {
            sum += stones[i] as usize;
            max = max.max(stones[i] as usize);
        }

        let p = (sum + max) / 2;

        let mut dp = vec![false; p + 1];
        dp[0] = true;

        for i in 0..stones.len() {
            for j in (stones[i] as usize..p+1).rev() {
                let temp =dp[j - stones[i] as usize] || dp[j];
                dp[j] = temp;
            }
        }

        let mid = sum / 2;
        let mut offset = 0;

        while !dp[mid + offset] && !dp[mid - offset] {
            offset += 1;
        }

        (2 * offset  + sum - 2 * mid) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_1049 {
        use super::*;

        #[test]
        fn test_1() {
            let stones = vec![2, 7, 4, 1, 8, 1];
            let result = 1;
            assert_eq!(Solution::last_stone_weight_ii(stones), result);
        }

        #[test]
        fn test_2() {
            let stones = vec![31, 26, 33, 21, 40];
            let result = 5;
            assert_eq!(Solution::last_stone_weight_ii(stones), result);
        }
    }
}
