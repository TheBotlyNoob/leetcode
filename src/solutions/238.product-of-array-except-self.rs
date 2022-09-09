/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 *
 * https://leetcode.com/problems/product-of-array-except-self/description/
 *
 * algorithms
 * Medium (64.49%)
 * Likes:    14427
 * Dislikes: 827
 * Total Accepted:    1.3M
 * Total Submissions: 2.1M
 * Testcase Example:  '[1,2,3,4]'
 *
 * Given an integer array nums, return an array answer such that answer[i] is
 * equal to the product of all the elements of nums except nums[i].
 *
 * The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
 * integer.
 *
 * You must write an algorithm that runs in O(n) time and without using the
 * division operation.
 *
 *
 * Example 1:
 * Input: nums = [1,2,3,4]
 * Output: [24,12,8,6]
 * Example 2:
 * Input: nums = [-1,1,0,-3,3]
 * Output: [0,0,9,0,0]
 *
 *
 * Constraints:
 *
 *
 * 2 <= nums.length <= 10^5
 * -30 <= nums[i] <= 30
 * The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
 * integer.
 *
 *
 *
 * Follow up: Can you solve the problem in O(1) extra space complexity? (The
 * output array does not count as extra space for space complexity analysis.)
 *
 */

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        let mut left = 1;
        let mut right = 1;
        for i in 0..nums.len() {
            res[i] *= left;
            left *= nums[i];
            res[nums.len() - 1 - i] *= right;
            right *= nums[nums.len() - 1 - i];
        }
        res
    }
}
// @lc code=end

#[test]
fn test() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
    assert_eq!(
        Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}
