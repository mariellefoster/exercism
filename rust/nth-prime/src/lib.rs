pub fn nth(n: u32) -> u32 {
    let mut prime_vec: Vec<u32> = vec![];
    prime_vec.push(2);
    let mut x = 3;
    while prime_vec.len() < (n as usize) {
        if prime_vec.iter().any(|&prime| x % prime == 0) {
            continue
        }
        prime_vec.push(x);
        x += 2;
    }
    prime_vec[n as usize]
}
