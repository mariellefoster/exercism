pub fn verse(n: i32) -> String {
    let mut ret = "".to_string();
    if n == 0 {
        ret = format!("No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.");
    } else if n == 1 {
        ret = format!("1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n");
    } else if n == 2 {
        ret = format!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1);
    } else if n > 1 {
        ret = format!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1);
    }
    ret
}

pub fn sing(start: i32, end: i32) -> String {
    let mut ret = "".to_string();
    if start == end {
        ret = verse(start)
    } else {
        ret = verse(end);
        for n in end+1..start+1 {
            ret = format!("{}\n{}", verse(n), ret);
        }
    }
    ret
}
