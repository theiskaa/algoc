// Two Sum - https://leetcode.com/problems/two-sum/

/* Problem Statement:
 *  Given an array of integers [nums] and an integer [target],
 *  return indices of the two numbers such that they add up to [target].
 *  You may assume that each input would have exactly one solution,
 *  and you may not use the same element twice.
 *  You can return the answer in any order.
 */

#![allow(dead_code)]

use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Eq, PartialEq, Hash)]
struct Input {
    nums: Vec<i32>,
    target: i32,
}

impl Input {
    pub fn new(nums: Vec<i32>, target: i32) -> Self {
        Self { nums, target }
    }
}

pub fn run() {
    let test_to_answer: HashMap<Input, Vec<i32>> = HashMap::from([
        (Input::new(vec![2, 7, 11, 15], 9), vec![0, 1]),
        (Input::new(vec![3, 2, 4], 6), vec![1, 2]),
        (Input::new(vec![3, 3], 6), vec![0, 1]),
    ]);

    for (input, mut expected) in test_to_answer {
        let mut result = two_sum_hash_map(input.nums, input.target);
        assert_eq!(result.sort(), expected.sort());
    }
}

/* Hash Map Solution
 * -----------------
 * Loop through the nums, take each [element]'s and [target]'s difference
 * Check differences existent at hash-map, if does exists, return current's index and founded value's
 * index.
 * If not exists, insert at hash-map, as value => index.
 */
fn two_sum_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut garbage: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        let diff = target - v;
        if let Some(&m) = garbage.get(&diff) {
            return vec![m as i32, i as i32];
        }

        garbage.insert(*v, i.try_into().unwrap());
    }

    vec![]
}

/* Brute Force Solution
 * --------------------
 * Create two nested loops by incremented indexes. -> [i] and [j].
 * Check nums' [i]th element's and nums' [j]th element's sum, if it equals to the target
 * return the [i] and [j] as vector.
 */
fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i.try_into().unwrap(), j.try_into().unwrap()];
            }
        }
    }

    vec![]
}
