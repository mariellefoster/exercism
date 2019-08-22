use std::cmp;

pub fn build_proverb(list: &[&str]) -> String {
    let mut ret = String::new();
    let mut dummy = String::new();
    if list.len() > 0 {
        for i in 0..list.len()-1 {
            ret = format!("{}For want of a {} the {} was lost.\n", ret, list[i], list[i+1]);
        }
    }
    match list.len() {
        0 => ret,
        _ => format!("{}And all for the want of a {}.", ret, list[0])
    }
}
