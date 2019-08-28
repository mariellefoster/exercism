pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut t = factors[0];
    let final_factors = [u32];
    for f in factors {
        // push all multiples
        while t < limit {
            let is_present = factors.iter().any(|t| t == &x);
            if !is_present {
                final_factors.push(t);
            }
            t = t + f;
        }
    }
    // while factor < limit {

    //     let is_present = factors.iter().any(|c| c.position == cell.position);
    // }
}
