use fastxfix::CommonStr;
use std::iter;
use ya_rand::encoding::{Base16, Encoder};
use ya_rand::{Generator, SecureGenerator, SecureRng};

/// HAHAHA FUNNY NUMBER HAHAHA
const STRING_COUNT: usize = 69;

fn main() {
    let mut rng = SecureRng::new();
    let v = Vec::from_iter(iter::repeat_n(
        rng.text::<Base16>(Base16::MIN_LEN),
        STRING_COUNT,
    ));
    match v.common_prefix_ref() {
        Some(s) => println!("common prefix of '{}' was randomly generated", s),
        None => println!("no common prefix was randomly generated"),
    }
}
