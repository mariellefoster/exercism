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

        if !is_back_bracket(character) {
            continue;
        } else {
            let popped_char = characters.pop();
            match (popped_char, character) {
                (Some('{'), '}') => (),
                (Some('['), ']') => (),
                (Some('('), ')') => (),
                (_, _) => characters.push(character)
            };
        }
    }

    if characters.len() != 0 {
        false
    } else {
        true
    }
}

pub fn is_back_bracket(b: char) -> bool {
    // turn this into "member of list?"
    match b {
        '}' => true,
        ')' => true,
        ']' => true,
        _ => false
    }
}
