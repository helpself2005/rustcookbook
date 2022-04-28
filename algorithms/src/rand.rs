use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub trait IRand {
    fn rand_i32() -> i32;

    fn rand_string() -> String;

    fn rand_pass() -> String;
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 10;

pub struct Rand {}

impl IRand for Rand {
    fn rand_i32() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen::<i32>()
    }

    fn rand_string() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    fn rand_pass() -> String {
        let mut rng = rand::thread_rng();

        (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}