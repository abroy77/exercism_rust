pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        // enforce limits on square index
        panic!("Square must be between 1 and 64");
    }

    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x))
}
