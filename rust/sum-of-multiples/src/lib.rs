// Take 3
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|n| factors.iter().any(|f| f > &0 && n%f == 0)).sum()
}


// Take 2
//     (1..limit).filter(|&n| factors.iter()
//                                   .filter(|&&fac| fac != 0)
//                                   .any(|&fac| n % fac == 0))
//                                   .sum()
//     }
// }

// Take 1
    // let mut vec = vec![];

    // for f in factors {
    //     let mut m = f;
    //     if f > &0u32 {
    //         while m < &limit {
    //             vec.push(m);
    //             let mut m = (m + f);
    //         }
    //     }
        
    // }
    // vec.sort_unstable();
    // vec.dedup();
    // vec.into_iter().sum()