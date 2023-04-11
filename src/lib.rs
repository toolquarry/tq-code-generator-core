use rand::{Rng, thread_rng};
use std::ops::Range;

pub fn generate_random_code(length: usize) -> String {
    let mut rng = thread_rng();
    let code: String = (0..length)
        .map(|_| {
            loop {
                let c = rng.gen_range(Range{start: b'A', end: b'z'+1}) as char;
                if c.is_ascii_alphabetic() {
                    break c;
                }
            }
        })
        .collect();
    code
}
