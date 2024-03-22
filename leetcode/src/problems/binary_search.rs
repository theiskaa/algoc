// Binary Search - https://leetcode.com/problems/binary-search

/* Problem Statement:
 * Given an array of integers nums which is sorted in ascending order,
 * and an integer target, write a function to search target in nums.
 * If target exists, then return its index. Otherwise, return -1.
 *
 * You must write an algorithm with O(log n) runtime complexity.
*/

/* Examples:
 *
 * Input: nums = [-1,0,3,5,9,12] & target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
*/

use std::collections::HashMap;

pub fn run() {
    let test_to_answer: HashMap<(Vec<i32>, i32), i32> = HashMap::from([
        ((vec![-1, 0, 3, 5, 9, 12], 9), 4),
        ((vec![-1, 0, 3, 5, 9, 12], 2), -1),
    ]);

    for (inputs, expected) in test_to_answer {
        let result: i32 = search(inputs.0.clone(), inputs.1.clone());
        assert_eq!(result, expected, "Where 0:{:?} and 1:{}", inputs.0, inputs.1);
    }
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0.0;
    let mut high = nums.len() as f32;

    while low < high {
        let middle: f32 = ((low + high) / 2.0).floor();
        let value = nums[middle as usize];
        if value == target {
            return middle as i32;
        }

        if value > target {
            high = middle.clone();
        } else {
            low = middle + 1.0
        }
    }

    -1
}
