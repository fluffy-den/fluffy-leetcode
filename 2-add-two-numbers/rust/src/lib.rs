#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;

        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;

        let mut n1 = l1;
        let mut n2 = l2;

        while n1.is_some() || n2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = n1 {
                sum += node.val;
                n1 = node.next;
            }

            if let Some(node) = n2 {
                sum += node.val;
                n2 = node.next;
            }

            carry = sum / 10;
            let digit = sum % 10;

            tail.next = Some(Box::new(ListNode {
                val: digit,
                next: None,
            }));
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

fn list_make(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut next: Option<Box<ListNode>> = None;
    for val in vals.iter().rev() {
        let cur = Box::new(ListNode { val: *val, next });
        next = Some(cur);
    }
    next
}

fn list_eq(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> bool {
    if l1.is_none() && l2.is_none() {
        return true;
    }
    if l1.is_none() != l2.is_none() {
        return false;
    }

    let mut c1 = &l1;
    let mut c2 = &l2;

    while !c1.is_none() && !c2.is_none() {
        if c1.as_ref().unwrap().val != c2.as_ref().unwrap().val {
            return false;
        }

        if !c1.is_none() {
            c1 = &c1.as_ref().unwrap().next;
        }
        if !c2.is_none() {
            c2 = &c2.as_ref().unwrap().next;
        }
    }

    true
}

#[test]
fn test_case_1() {
    let l1 = list_make(&[2, 4, 3]);
    let l2 = list_make(&[5, 6, 4]);
    assert_eq!(Solution::add_two_numbers(l1, l2), list_make(&[7, 0, 8]));
}

#[test]
fn test_case_2() {
    let l1 = list_make(&[0]);
    let l2 = list_make(&[0]);
    assert_eq!(Solution::add_two_numbers(l1, l2), list_make(&[0]));
}

#[test]
fn test_case_3() {
    let l1 = list_make(&[9, 9, 9, 9, 9, 9, 9]);
    let l2 = list_make(&[9, 9, 9, 9]);
    assert_eq!(
        Solution::add_two_numbers(l1, l2),
        list_make(&[8, 9, 9, 9, 0, 0, 0, 1])
    );
}

