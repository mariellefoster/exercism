pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    let len = message.len();
    if len == 0 {
        return "Fine. Be that way!"
    }
    let question = if let "?" = &message[len-1..] {
        true
    } else {
        false
    };

    let mut lowercase = false;
    let mut uppercase = false;
    for c in message.chars(){
        if c.is_lowercase() {
            lowercase = true;
        }
        if c.is_uppercase() {
            uppercase = true;
        }
    };

    if !lowercase && uppercase {
        if question {
            return "Calm down, I know what I'm doing!"
        }
        return "Whoa, chill out!"
    } else if question {
        return "Sure."
    }
    return "Whatever."

    // if question "Sure."

    // if yelling in caps "Whoa, chill out!"

    // if yelling question "Calm down, I know what I'm doing!"

    // if nothing "Fine. Be that way!"

    // all else "Whatever."
}
