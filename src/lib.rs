use rand::{Rng, thread_rng};
use std::ops::Range;

pub fn generate_random_code(length: usize) -> String {
    let mut rng = thread_rng();
    let code: String = (0..length)
        .map(|_| rng.gen_range(Range{start: b'A', end: b'Z'+1}) as char)
        .collect();
    code
}
