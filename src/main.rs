use fastxfix::CommonStr;
use ya_rand::encoding::{Base16, Encoder};
use ya_rand::{Generator, SecureGenerator, SecureRng};

/// HAHAHA FUNNY NUMBER HAHAHA
const STRING_COUNT: usize = 69420;

fn main() {
    let mut rng = SecureRng::new();
    let mut v = vec![String::default(); STRING_COUNT];
    v.fill_with(|| rng.text::<Base16>(Base16::MIN_LEN));
    match v.common_prefix_ref() {
        Some(s) => println!("common prefix of '{}' was randomly generated", s),
        None => println!("no common prefix was randomly generated"),
    }
}
