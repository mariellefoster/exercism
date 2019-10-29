pub fn brackets_are_balanced(string: &str) -> bool {
    let mut characters = Vec::new();

    for character in string.chars() {
        println!("{:?}", characters);
        match character {
            '{' => characters.push(character), //add to list
            '[' => characters.push(character), //add to list
            '(' => characters.push(character), //add to list
            _ => (),
        }

        if character == '}' {
            let popped_char = characters.pop();
            if popped_char != Some('{') {
                characters.push(character);
            }
        } //pop from list
        else if character == ']' {
            let popped_char = characters.pop();
            if popped_char != Some('[') {
                characters.push(character);
            }
        }
        else if character == ')' {
            let popped_char = characters.pop();
            if popped_char != Some('(') {
                characters.push(character);
            }
        }
    }

    if characters.len() != 0 {
        false
    } else {
        true
    }
}
