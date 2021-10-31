#![allow(dead_code)]

struct Solution;

use crate::data_structures::linked_lists::ListNode;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::merge(l1, l2, 0, ListNode::new(-1))
    }

    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
        mut carry: i32,
        mut result: ListNode,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        if let Some(n1) = l1 {
            carry += n1.val;
            l1 = n1.next;
        }

        if let Some(n2) = l2 {
            carry += n2.val;
            l2 = n2.next;
        }

        result.val = carry % 10;
        result.next = Solution::merge(l1, l2, carry / 10, ListNode::new(-1));

        Some(Box::new(result))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::data_structures::linked_lists::{
        create_linked_list_from_vec, create_vec_from_linked_list, List,
    };

    #[test]
    pub fn test_basic_test_case1() {
        let v1 = vec![2, 4, 3];
        let v2 = vec![5, 6, 4];
        let expected = vec![7, 0, 8];

        let list1 = create_linked_list_from_vec(&v1);
        let list2 = create_linked_list_from_vec(&v2);

        let result = Solution::add_two_numbers(list1.head, list2.head);

        let result_as_vec = create_vec_from_linked_list(List { head: result });

        assert_eq!(result_as_vec, expected);
    }
}
