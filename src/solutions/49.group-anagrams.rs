/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 *
 * https://leetcode.com/problems/group-anagrams/description/
 *
 * algorithms
 * Medium (65.54%)
 * Likes:    11522
 * Dislikes: 363
 * Total Accepted:    1.6M
 * Total Submissions: 2.4M
 * Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
 *
 * Given an array of strings strs, group the anagrams together. You can return
 * the answer in any order.
 *
 * An Anagram is a word or phrase formed by rearranging the letters of a
 * different word or phrase, typically using all the original letters exactly
 * once.
 *
 *
 * Example 1:
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * Example 2:
 * Input: strs = [""]
 * Output: [[""]]
 * Example 3:
 * Input: strs = ["a"]
 * Output: [["a"]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= strs.length <= 10^4
 * 0 <= strs[i].length <= 100
 * strs[i] consists of lowercase English letters.
 *
 *
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        for s in strs {
            let mut s_bytes = s.clone().into_bytes();
            s_bytes.sort_unstable();
            map.entry(s_bytes).or_insert_with(Vec::new).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}
// @lc code=end

#[test]
fn test_case() {
    fn assert_unordered(mut a: Vec<Vec<String>>, mut b: Vec<Vec<String>>) {
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }
    assert_unordered(
        Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]),
        vec![
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()],
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
        ],
    );
}
