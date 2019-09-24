pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2
    }
    let mut prime = 3;
    for _ in 1..(n) {
        loop {
            prime += 2;
            if is_prime(prime) {
                break;
            }
        }
    }
    prime
}

pub fn is_prime(n: u32) -> bool {
    let s = (n as f64).sqrt() as u32 + 1;
    for i in 3..s {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// First Attempt
// pub fn nth(n: u32) -> u32 {
//     let mut prime_vec: Vec<u32> = vec![];
//     prime_vec.push(2);
//     let mut x = 3;
//     while prime_vec.len() < (n as usize) {
//         if prime_vec.iter().any(|&prime| x % prime == 0) {
//             continue
//         }
//         prime_vec.push(x);
//         x += 2;
//     }
//     prime_vec[n as usize]
// }
