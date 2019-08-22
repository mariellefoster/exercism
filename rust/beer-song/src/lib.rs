pub fn verse(n: i32) -> String {
    match n {
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1)
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut ret = "".to_string();
    if start == end {
        ret = verse(start);
    } else {
        ret = verse(end);
        for n in end+1..start+1 {
            ret = format!("{}\n{}", verse(n), ret);
        }
    }
    ret
}
