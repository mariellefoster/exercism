use std::vec::Vec;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut vec = vec![];

    for f in factors {
        let mut m = f;
        while m < &limit {
            vec.push(m);
            let mut m = (m + f);
        }
    }
    vec.sort_unstable();
    vec.dedup();
    vec.into_iter().sum()
    // while factor < limit {

    //     let is_present = factors.iter().any(|c| c.position == cell.position);
    // }
}
