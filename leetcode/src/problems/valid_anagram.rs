// Valid Anagram - https://leetcode.com/problems/valid-anagram

/* Problem Statement:
 * An Anagram is a word or phrase formed by rearranging
 * the letters of a different word or phrase, typically
 * using all the original letters exactly once.
 */

/*
 * Examples:
 *
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * Explanation: "sorted s & t would be same, so it's anagram"
 *
 * Input: s = "rat", t = "car"
 * Output: false
 * Explanation: "sorted s & t would not be same, so it's not anagram"
 */

use std::collections::HashMap;

pub fn run() {
    let test_to_answer: HashMap<(String, String), bool> = HashMap::from([
        ((String::from("anagram"), String::from("nagaram")), true),
        ((String::from("rat"), String::from("car")), true),
    ]);

    for (inputs, expected) in test_to_answer {
        let result: bool = is_anagram(inputs.0.clone(), inputs.1.clone());
        assert_eq!(result, expected, "Where 0:{} and 1:{}", inputs.0, inputs.1);
    }
}

/* Sorting
 *  --------
 * Check both parameters' length. If they aren't equal these values couldn't be anagram.
 * Sort both parameters via "increasing" order. And check each match. If "nth" indexed value of
 * [s] sorted slice, doesn't equals to the "nth" indexed value of [t] sorted slice, it means that
 * they are not an anagrams of each other.
 * If cases doesn't match, return true. We founded anagram.
*/
fn is_anagram(s: String, t: String) -> bool {
    let mut s_sorted: Vec<char> = s.chars().collect();
    let mut t_sorted: Vec<char> = t.chars().collect();
    s_sorted.sort();
    t_sorted.sort();

    return s_sorted == t_sorted;
}
