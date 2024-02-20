use ansi_term::Colour;
use algorithms::entities::user::User;
use algorithms::leetcode::vecsum::two_sum;

fn main() {
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
}


