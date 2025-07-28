pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut new_word = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            new_word = true;
        } else if new_word {
            result.extend(c.to_uppercase());
            new_word = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

