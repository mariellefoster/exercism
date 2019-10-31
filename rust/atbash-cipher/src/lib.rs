/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    // read in string as characters
    // create new string to return
    result = Vec::new();
    // convert to lowercase
    // strip whitespace and punctuation
    let clean_plain = plain.split_whitespace().collect();
    for character in clean_plain.chars() {
        if !character.is_ascii() || character.is_ascii_punctuation() {
            continue;
        }
        let c = AsciiChar::from_ascii(character).as_byte();
        c 
    }
    

    // loop through characters, converting to ascii
    // if it's a number, just keep the number

    // convert ascii char to its opposite. 97-122 is the
    // current range for lower case letters
    // so minus 97, add -25, take abs value, use that

    // add 97, convert to ascii and then character
    // add to string
    // create chunker function which inserts spaces every five
    // chars
    unimplemented!("Encoding of {:?} in Atbash cipher.", plain);
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
