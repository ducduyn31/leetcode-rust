#![allow(dead_code)]

use std::collections::HashMap;

/**
 * [648] Replace Words
 *
 * In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word derivative. For example, when the root "help" is followed by the word "ful", we can form a derivative "helpful".
 * Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the derivatives in the sentence with the root forming it. If a derivative can be replaced by more than one root, replace it with the root that has the shortest length.
 * Return the sentence after the replacement.
 *  
 * <strong class="example">Example 1:
 *
 * Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
 * Output: "the cat was rat by the bat"
 *
 * <strong class="example">Example 2:
 *
 * Input: dictionary = ["a","b","c"], sentence = "aadsfasf absbs bbab cadsfafs"
 * Output: "a a b c"
 *
 *  
 * Constraints:
 *
 * 	1 <= dictionary.length <= 1000
 * 	1 <= dictionary[i].length <= 100
 * 	dictionary[i] consists of only lower-case letters.
 * 	1 <= sentence.length <= 10^6
 * 	sentence consists of only lower-case letters and spaces.
 * 	The number of words in sentence is in the range [1, 1000]
 * 	The length of each word in sentence is in the range [1, 1000]
 * 	Every two consecutive words in sentence will be separated by exactly one space.
 * 	sentence does not have leading or trailing spaces.
 *
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/replace-words/
// discuss: https://leetcode.com/problems/replace-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let trie = Self::build_trie(dictionary);

        sentence
            .split_whitespace()
            .map(|word| {
                let mut current = &trie;
                let mut replacement = None;

                for (i, c) in word.chars().enumerate() {
                    if let Some(child_node) = current.children.get(&c) {
                        current = child_node;

                        if current.is_end_of_word {
                            replacement = Some(&word[..i + 1]);
                            break;
                        }
                    } else {
                        break;
                    }
                }

                replacement.unwrap_or(word)
            })
            .collect::<Vec<&str>>()
            .join(" ")
    }

    fn build_trie(dictionary: Vec<String>) -> TrieNode {
        let mut root = TrieNode::new();

        for word in dictionary {
            let mut current = &mut root;
            for c in word.chars() {
                current = current.children.entry(c).or_insert_with(TrieNode::new);
            }
            current.is_end_of_word = true;
        }

        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    mod test_648 {
        use super::*;

        #[test]
        fn test_1() {
            let dictionary = vec!["cat".to_string(), "bat".to_string(), "rat".to_string()];
            let sentence = "the cattle was rattled by the battery".to_string();
            let result = "the cat was rat by the bat".to_string();
            assert_eq!(Solution::replace_words(dictionary, sentence), result);
        }

        #[test]
        fn test_2() {
            let dictionary = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let sentence = "aadsfasf absbs bbab cadsfafs".to_string();
            let result = "a a b c".to_string();
            assert_eq!(Solution::replace_words(dictionary, sentence), result);
        }
    }
}

