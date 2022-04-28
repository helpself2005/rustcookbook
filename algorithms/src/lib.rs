pub mod entities;
pub mod rand;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::add;
    use crate::rand::{Rand, IRand};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn add_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn rand_i32_test() {
        let result = Rand::rand_i32();
        assert_eq!(result, 1);
    }


    #[test]
    fn rand_string_test() {
        let result = Rand::rand_string();
        println!("{}", result);
    }
}
