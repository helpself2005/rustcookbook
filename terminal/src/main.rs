use std::collections::HashMap;
use ansi_term::Colour;
use algorithms::entities::user::User;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));

    let nums = vec![2,7,11,15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{}", result.len());

    let user = User{id :"uuid".to_string(), name : "ddd".to_string(), age :9};

    println!("{}", user)

}


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
    panic!("not found")
}