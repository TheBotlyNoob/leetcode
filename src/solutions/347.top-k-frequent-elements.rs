/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 *
 * https://leetcode.com/problems/top-k-frequent-elements/description/
 *
 * algorithms
 * Medium (64.87%)
 * Likes:    10971
 * Dislikes: 411
 * Total Accepted:    1.1M
 * Total Submissions: 1.8M
 * Testcase Example:  '[1,1,1,2,2,3]\n2'
 *
 * Given an integer array nums and an integer k, return the k most frequent
 * elements. You may return the answer in any order.
 *
 *
 * Example 1:
 * Input: nums = [1,1,1,2,2,3], k = 2
 * Output: [1,2]
 * Example 2:
 * Input: nums = [1], k = 1
 * Output: [1]
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 * k is in the range [1, the number of unique elements in the array].
 * It is guaranteed that the answer is unique.
 *
 *
 *
 * Follow up: Your algorithm's time complexity must be better than O(n log n),
 * where n is the array's size.
 *
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut map = map.iter().collect::<Vec<_>>();
        map.sort_unstable_by_key(|(_, v)| std::cmp::Reverse(*v));
        map.iter().take(k as usize).map(|(k, _)| **k).collect()
    }
}
// @lc code=end

#[test]
fn test() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![1, 2]
    );
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
