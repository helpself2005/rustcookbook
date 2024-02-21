use ansi_term::Colour;
use algorithms::entities::user::User;
use algorithms::leetcode::listnode::ListNode;
use algorithms::leetcode::{add_two_listnode,add_two_numbers};

fn main() {
    /*
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));

    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result : Vec<i32> = two_sum(nums, target);
    println!("{:?}", result);
    let user = User { id: "uuid".to_string(), name: "ddd".to_string(), age: 9 };

    println!("{}", user)
    */

    let l1 = Some(Box::new(ListNode{val: 2, next: Some(Box::new(ListNode{val: 4,
            next: Some(Box::new(ListNode{val: 3, next:None}))}))}));
    let l2 = Some(Box::new(ListNode{val: 5, next: Some(Box::new(ListNode{val: 6,
        next: Some(Box::new(ListNode{val: 4, next:None}))}))}));

    let l3 = add_two_numbers(l1, l2);
    //let l3 = add_two_listnode(l1, l2);
    println!("{:?}", l3)
}


