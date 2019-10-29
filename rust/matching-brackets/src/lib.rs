pub fn brackets_are_balanced(string: &str) -> bool {
    let mut characters = Vec::new();

    for character in string.chars() {
        match character {
            '{' => characters.push(character), //add to list
            '[' => characters.push(character), //add to list
            '(' => characters.push(character), //add to list
            _ => continue,
        }

        if character == '}' {
            let popped_char = characters.pop();
            if popped_char != Some('{') {
                break;
            }
        } //pop from list
        else if character == ']' {
            let popped_char = characters.pop();
            if popped_char != Some('[') {
                break;
            }
        }
        else if character == ')' {
            let popped_char = characters.pop();
            if popped_char != Some('(') {
                break;
            }
        }
    }

    if characters.len() != 0 {
        false
    } else {
        true
    }
}
