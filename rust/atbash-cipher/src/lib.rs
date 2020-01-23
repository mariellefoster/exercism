/// "Helper" with the Atbash cipher.
pub fn helper(c : char) -> Option<char> {
    // let c = c1 as i16;
    match c {
        '0'...'9' => Some(c),
        'a' ... 'z' => Some((b'a' + b'z' - c as u8) as char),
        'A' ... 'Z'  => Some((b'a' + b'Z' - c as u8) as char),
        _ => None,
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::from("");
    let mut f = 0;

    plain.chars()
         .filter_map(helper)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter_map(helper).collect::<Vec<_>>()
        .chunks(5).collect::<Vec<_>>()
        .join(&' ').iter().cloned().collect()
}