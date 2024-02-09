// Best Time To Buy And Sell Stock - https://leetcode.com/problems/best-time-to-buy-and-sell-stock

/* Problem Statement:
 * You are given an array prices where prices[i] is the price of a given
 * stock on the [i]'th day.
 * You want to maximize your profit by choosing a single day to buy one
 * stock and choosing a different day in the future to sell that stock.
 * Return the maximum profit you can achieve from this transaction.
 * If you cannot achieve any profit, return 0.
*/

/* Examples:
 *
 * Input: prices = [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transactions are done and the max profit = 0.
 *
 * Input: prices = [7,1,5,3,6,4]
 * Output: 5
 * Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
               Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
*/

#![allow(dead_code)]

use std::collections::HashMap;

pub fn run() {
    let test_to_answer: HashMap<Vec<i32>, i32> = HashMap::from([
        (vec![7, 1, 5, 3, 6, 4], 5),
        (vec![7, 6, 4, 3, 1], 0),
        (vec![1, 3, 0, 2, 2, 9], 9),
    ]);

    for (input, expected) in test_to_answer {
        let result = max_profit(input);
        assert_eq!(result, expected);
    }
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    let mut left: usize = 0;
    let mut right: usize = 1;

    while right < prices.len() {
        if prices[left] < prices[right] {
            let profit = prices[right] - prices[left];
            if profit > max {
                max = profit;
            }
        } else {
            left = right;
        }

        right += 1;
    }

    max
}
