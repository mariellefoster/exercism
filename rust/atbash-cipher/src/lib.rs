/// "Helper" with the Atbash cipher.
pub fn helper(c : i16) -> char {
    match c {
        97 ... 123 => ((c - 122).abs() as u8 + 97) as char,
        65 ... 91  => ((c - 90).abs() as u8 + 97) as char,
        _ => (c as u8) as char,
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::from("");
    let mut f = 0;
    for character in plain.chars() {
        if !character.is_ascii() 
                || character.is_ascii_punctuation()
                || character.is_ascii_whitespace() {
            continue;
        }
        let c = character as i16;
        let d = helper(c);
        f += 1;
        result.push(d);
        if f % 5 == 0 {
            result.push(' ');
        }
    }
    result.trim_end().to_string()
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