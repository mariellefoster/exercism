pub fn reply(message: &str) -> &str {
    let len = message.len();
    let question = if let "?" = &message[len-1..] {
        true
    } else {
        false
    };

    let mut yelling = true;
    for c in message.chars(){
        if c.is_lowercase() {
            yelling = false;
        }
    };

    if yelling {
        if question {
            return "Calm down, I know what I\'m doing!"
        }
        return "Whoa, chill out!"
    } else if question {
        return "Sure."
    } else if message == "" {
        return "Fine. Be that way!"
    }
    return "Whatever."
    

    // | message | message[message.len()-1] == "?";

    // let yelling = boolean function

    // if question "Sure."

    // if yelling in caps "Whoa, chill out!"


    // if yelling question "Calm down, I know what I'm doing!"


    // if nothing "Fine. Be that way!"


    // all else "Whatever."

    // use pattern matching?
}
