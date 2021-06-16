pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_brackets = 0;
    let mut closed_brackets = 0;
    for x in string.chars() {
        if x == '{' || x == '(' || x == '[' {
            open_brackets += 1;
        }

        if x == '}' || x == ')' || x == ']' {
            closed_brackets += 1;
        }
    }

    if open_brackets - closed_brackets == 0 {
        true
    } else {
        false
    }
}
