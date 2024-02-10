// Valid Palindrome - https://leetcode.com/problems/valid-palindrome

/* Problem Statement:
 * A phrase is a palindrome if, after converting all uppercase letters
 * into lowercase letters and removing all non-alphanumeric characters,
 * it reads the same forward and backward. Alphanumeric characters include
 * letters and numbers.
 *
 * Given a string s, return true if it is a palindrome, or false otherwise.
 */

/* Examples:
 *
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 *
 * Input: s = "race car"
 * Output: true
 * Explanation: "racecar" is a palindrome.
 */

use std::collections::HashMap;

pub fn run() {
    let test_to_answer: HashMap<String, bool> = HashMap::from([
        (String::from("race a car"), false),
        (String::from("I'm me"), false),
        (String::from("race car"), true),
        (String::from("A man, a plan, a canal: Panama"), true),
        (String::from(" "), true),
    ]);

    for (input, expected) in test_to_answer {
        let result = is_palindrome(input);
        assert_eq!(result, expected);
    }
}

fn is_palindrome(s: String) -> bool {
    let alphanumeric: String = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
    let reversed: String = alphanumeric.chars().rev().collect();
    alphanumeric == reversed
}
