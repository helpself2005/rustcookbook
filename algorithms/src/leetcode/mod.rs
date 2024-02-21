//! leetcode mod

pub mod listnode;

use std::collections::HashMap;
use crate::leetcode::listnode::ListNode;


/// The `two_sum` set target value search from vector.
///
/// return vector index
///
/// Example:
///
/// ```rust
/// # use algorithms::leetcode::two_sum;
///  let nums = vec![2, 7, 11, 15];
///  let target = 9;
///  let result : Vec<i32> = two_sum(nums, target);
/// assert_eq!(result.len(), 2);
/// ```
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());

    for i in 0..nums.len() {
        if let Some(k) = map.get(&(target - nums[i])) {
            if *k != i {
                return vec![*k as i32,  i as i32];
            }
        }
        map.insert(nums[i], i);
    }
    return vec![]
}


/// The `add_two_listnode` add two listnode.
///
/// return listnode
///
/// Example:
///
/// ```rust
/// # use algorithms::leetcode::add_two_listnode;
/// # use algorithms::leetcode::listnode::ListNode;
/// let l1 = Some(Box::new(ListNode{val: 2, next: Some(Box::new(ListNode{val: 4,next: Some(Box::new(ListNode{val: 3, next:None}))}))}));
/// let l2 = Some(Box::new(ListNode{val: 5, next: Some(Box::new(ListNode{val: 6,next: Some(Box::new(ListNode{val: 4, next:None}))}))}));
/// let l3 = add_two_listnode(l1, l2);
/// println!("{:?}", l3)
/// ```
pub fn add_two_listnode(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two(&l1, &l2, 0)
}


// l1 和 l2 为当前遍历的节点，carry 为进位
fn add_two(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => { // 递归边界：l1 和 l2 都是空节点
            if carry == 0 {
                return None;
            }
            // 进位了，额外创建一个节点
            Some(Box::new(ListNode::new(carry)))
        }
        (None, Some(n2)) => add_two(l2, l1, carry),
        (Some(n1), None) => {
            let mut l1 = n1.clone();
            let mut sum = carry + l1.val; // 节点值和进位加在一起
            l1.val = sum % 10; // 每个节点保存一个数位
            l1.next = add_two(&n1.next, &None, sum / 10); // 进位
            Some(l1)
        }
        (Some(n1), Some(n2)) => {
            let mut l1 = n1.clone();
            let mut l2 = n2.clone();
            let mut sum = carry + l1.val + l2.val; // 节点值和进位加在一起
            l1.val = sum % 10; // 每个节点保存一个数位
            l1.next = add_two(&l1.next, &l2.next, sum / 10); // 进位
            Some(l1)
        }
    }
}