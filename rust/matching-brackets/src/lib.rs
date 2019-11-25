pub fn brackets_are_balanced(string: &str) -> bool {
    let mut characters = Vec::new();

    for character in string.chars() {
        match character {
            '{' | '[' | '(' => characters.push(character),
            '}' => if characters.pop() != Some('{') { return false },
            ']' => if characters.pop() != Some('[') { return false },
            ')' => if characters.pop() != Some('(') { return false },
            _ => (),
        }
    }

    characters.is_empty()
}