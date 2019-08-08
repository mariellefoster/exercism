pub fn raindrops(n: u32) -> String {
    // If the number has 3 as a factor, output 'Pling'.
    let mut s : String = "".to_string();
    if n % 3 == 0 {
        s.push_str("Pling");
    }
    // If the number has 5 as a factor, output 'Plang'.
    if n % 5 == 0 {
        s.push_str("Plang");
    }
    // If the number has 7 as a factor, output 'Plong'.
    if n % 7 == 0 {
        s.push_str("Plong");
    }
    if s.len() > 0 {
        return s
    }
    // If the number does not have 3, 5, or 7 as a factor,
    //   just pass the number's digits straight through.
    n.to_string()
}
