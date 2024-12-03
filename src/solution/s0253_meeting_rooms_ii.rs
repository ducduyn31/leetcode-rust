#![allow(dead_code)]

use std::collections::BinaryHeap;

/**
 * [253] Meeting Rooms II
 *
 * Given an array of meeting time intervals intervals where intervals[i] = [starti, endi], return the minimum number of conference rooms required.
 *  
 * <strong class="example">Example 1:
 * Input: intervals = [[0,30],[5,10],[15,20]]
 * Output: 2
 *
 * <strong class="example">Example 2:
 * Input: intervals = [[7,10],[2,4]]
 * Output: 1
 *
 * Constraints:
 *
 * 	1 <= intervals.length <= 104
 * 	0 <= starti < endi <= 106
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/meeting-rooms-ii/
// problem: https://leetcode.com/problems/meeting-rooms-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|v| v[0]);
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for interval in intervals {
            if let Some(&top) = heap.peek() {
                if top.abs() <= interval[0] {
                    heap.pop();
                }
            }
            heap.push(-interval[1]);
        }
        heap.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_253 {
        use super::*;

        #[test]
        fn test_1() {
            let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
            let result = 2;
            assert_eq!(Solution::min_meeting_rooms(intervals), result);
        }

        #[test]
        fn test_2() {
            let intervals = vec![vec![7, 10], vec![2, 4]];
            let result = 1;
            assert_eq!(Solution::min_meeting_rooms(intervals), result);
        }
    }
}

