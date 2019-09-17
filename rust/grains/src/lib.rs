pub fn square(s: u32) -> u64 {
    if s > 0 && s < 65{
        2u64.pow(s-1)
    } else {
        panic!("Square must be between 1 and 64");
        0
    }
}

pub fn total() -> u64 {
    18_446_744_073_709_551_615
}
