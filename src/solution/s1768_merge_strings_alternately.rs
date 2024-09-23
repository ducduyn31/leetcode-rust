#![allow(dead_code)]

/**
 * [1768] Merge Strings Alternately
 *
 * You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
 * 
 * Return the merged string.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * 
 * Input: word1 = "abc", word2 = "pqr"
 * Output: "apbqcr"
 * Explanation: The merged string will be merged as so:
 * word1:  a   b   c
 * word2:    p   q   r
 * merged: a p b q c r
 * 
 * 
 * <strong class="example">Example 2:
 * 
 * 
 * Input: word1 = "ab", word2 = "pqrs"
 * Output: "apbqrs"
 * Explanation: Notice that as word2 is longer, "rs" is appended to the end.
 * word1:  a   b 
 * word2:    p   q   r   s
 * merged: a p b q   r   s
 * 
 * 
 * <strong class="example">Example 3:
 * 
 * 
 * Input: word1 = "abcd", word2 = "pq"
 * Output: "apbqcd"
 * Explanation: Notice that as word1 is longer, "cd" is appended to the end.
 * word1:  a   b   c   d
 * word2:    p   q 
 * merged: a p b q c   d
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	1 <= word1.length, word2.length <= 100
 * 	word1 and word2 consist of lowercase English letters.
 * 
 */


pub struct Solution {}

// problem: https://leetcode.com/problems/merge-strings-alternately/
// discuss: https://leetcode.com/problems/merge-strings-alternately/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut word1_iter = word1.chars();
        let mut word2_iter = word2.chars();
        loop {
            match (word1_iter.next(), word2_iter.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c1), None) => {
                    result.push(c1);
                    result.push_str(word1_iter.as_str());
                    break;
                }
                (None, Some(c2)) => {
                    result.push(c2);
                    result.push_str(word2_iter.as_str());
                    break;
                }
                (None, None) => {
                    break;
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_1768 {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
                "apbqcr".to_string()
            );
        }

        #[test]
        fn test_2() {
            assert_eq!(
                Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
                "apbqrs".to_string()
            );
        }

        #[test]
        fn test_3() {
            assert_eq!(
                Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
                "apbqcd".to_string()
            );
        }
    }
}