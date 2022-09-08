/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 *
 * https://leetcode.com/problems/valid-anagram/description/
 *
 * algorithms
 * Easy (62.57%)
 * Likes:    6864
 * Dislikes: 238
 * Total Accepted:    1.6M
 * Total Submissions: 2.6M
 * Testcase Example:  '"anagram"\n"nagaram"'
 *
 * Given two strings s and t, return true if t is an anagram of s, and false
 * otherwise.
 *
 * An Anagram is a word or phrase formed by rearranging the letters of a
 * different word or phrase, typically using all the original letters exactly
 * once.
 *
 *
 * Example 1:
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * Example 2:
 * Input: s = "rat", t = "car"
 * Output: false
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length, t.length <= 5 * 10^4
 * s and t consist of lowercase English letters.
 *
 *
 *
 * Follow up: What if the inputs contain Unicode characters? How would you
 * adapt your solution to such a case?
 *
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn is_anagram(mut s: String, mut t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s = unsafe { s.as_bytes_mut() };
        let t = unsafe { t.as_bytes_mut() };
        s.sort_unstable();
        t.sort_unstable();
        s == t
    }
}
// @lc code=end
