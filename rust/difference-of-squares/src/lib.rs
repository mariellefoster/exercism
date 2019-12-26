pub fn square_of_sum(n: u32) -> u32 {
    let sum = n*(n+1) / 2;
    u32::pow(sum, 2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // n^3/3 + n^2/2 + n/6
    (n*(n+1)*(2*n+1))/6
}


pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
