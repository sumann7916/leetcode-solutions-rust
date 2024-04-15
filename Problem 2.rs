// Title: Add Two Numbers
// Problem: You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result_head: Option<Box<ListNode>> = None;
        let mut result_tail = &mut result_head;
        
        let mut carry = 0;

        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() || carry != 0 {
            let val1 = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => 0,
            };
            let val2 = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => 0,
            };
            
            let total_sum = val1 + val2 + carry;
            carry = total_sum / 10;
            let remainder = total_sum % 10;
            
            let new_node = Some(Box::new(ListNode {
                val: remainder,
                next: None,
            }));
            
            *result_tail = new_node;
            result_tail = &mut result_tail.as_mut().unwrap().next;
        }

        result_head
    }
}

