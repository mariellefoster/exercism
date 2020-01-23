/// "Helper" with the Atbash cipher.
pub fn helper(c : i16) -> Option<char> {
    // let c = c1 as i16;
    match c {
        '0'...'9' => Some(c),
        b'a' ... b'z' => Some((b'a' + b'z' - c as u8) as char),
        b'A' ... b'Z'  => Some((b'a' + b'Z' - c as u8) as char),
        _ => None,
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::from("");
    let mut f = 0;

    plain.chars().filter(|helper(c)| c.is_ascii()
                        || !c.is_ascii_punctuation()
                        || !c.is_ascii_whitespace()).collect()

    // for character in plain.chars() {
    //     if !character.is_ascii() 
    //             || character.is_ascii_punctuation()
    //             || character.is_ascii_whitespace() {
    //         continue;
    //     }
    //     let c = character as i16;
    //     let d = helper(c);
    //     f += 1;
    //     result.push(d);
    //     if f % 5 == 0 {
    //         result.push(' ');
    //     }
    // }
    // result.trim_end().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut result = String::from("");
    for character in cipher.chars() {
        if character.is_ascii_whitespace() {
            continue;
        }
        let c = character as i16;
        let d = helper(c);
        result.push(d);
    }
    result
}
