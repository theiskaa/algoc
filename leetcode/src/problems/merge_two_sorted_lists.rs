// Merge Two Sorted Lists - https://leetcode.com/problems/merge-two-sorted-lists

/* Problem Statement:
 * You are given the heads of two sorted linked lists list1 and list2.
 * Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
 * Return the head of the merged linked list.
 */

#![allow(dead_code)]

use std::collections::HashMap;

pub fn run() {
    let mut test_to_answer: HashMap<
        (Option<Box<ListNode>>, Option<Box<ListNode>>),
        Option<Box<ListNode>>,
    > = HashMap::new();

    // Test cases
    test_to_answer.insert(
        (None, Some(Box::new(ListNode::leaf(1)))),
        Some(Box::new(ListNode::leaf(1))),
    );
    test_to_answer.insert(
        (Some(Box::new(ListNode::leaf(1))), None),
        Some(Box::new(ListNode::leaf(1))),
    );
    test_to_answer.insert(
        (Some(Box::new(ListNode::leaf(1))), None),
        Some(Box::new(ListNode::leaf(1))),
    );
    test_to_answer.insert(
        (
            Some(Box::new(ListNode::new(
                1,
                ListNode::new(2, ListNode::leaf(4)),
            ))),
            Some(Box::new(ListNode::new(
                1,
                ListNode::new(3, ListNode::leaf(4)),
            ))),
        ),
        Some(Box::new(ListNode::new(
            1,
            ListNode::new(
                1,
                ListNode::new(2, ListNode::new(3, ListNode::new(4, ListNode::leaf(4)))),
            ),
        ))),
    );

    for (key, expected_value) in test_to_answer {
        assert_eq!(merge_two_lists(key.0, key.1), expected_value);
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Self) -> Self {
        ListNode {
            next: Some(Box::new(next)),
            val,
        }
    }

    #[inline]
    fn leaf(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (None, None) => None,
        (Some(l1), Some(l2)) => match l1.val <= l2.val {
            true => Some(Box::new(ListNode {
                val: l1.val,
                next: merge_two_lists(l1.next, Some(l2)),
            })),
            false => Some(Box::new(ListNode {
                val: l2.val,
                next: merge_two_lists(Some(l1), l2.next),
            })),
        },
    }
}
