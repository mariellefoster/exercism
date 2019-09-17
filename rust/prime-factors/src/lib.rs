pub fn factors(n: u64) -> Vec<u64> {
    // can you do something like 1...n/2.iter().collect( |s| where n % s == 0 )
    let mut s = n;
    let mut vec = vec![];
    if s > 1 {
        for i in 2..s+1 {
            while s % i == 0 {
                vec.push(i);
                s = s / i;
            }
            if i > s/2 {
                if s != 1 {
                    vec.push(s)
                }
                break;
            }
        }
    }
    vec
}
