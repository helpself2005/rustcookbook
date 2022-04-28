use algorithms::entities::user::User;
use algorithms::add;
use algorithms::rand::{Rand, IRand};

fn main() {
    let user = User::new(String::from("shipl"), 12);

    let result = add(2, 2);

    let rand_i32 = Rand::rand_i32();

    let rand_string = Rand::rand_string();

    let rand_pass = Rand::rand_pass();

    println!("Hello, world! {}, {:?}", result, user);

    println!("rand_i32 {}", rand_i32);

    println!("rand_string {}", rand_string);

    println!("rand_pass {}", rand_pass);


}
