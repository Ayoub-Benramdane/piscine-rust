pub fn talking(text: &str) -> &str {
    let trim_text = text.trim();
    if trim_text.is_empty() {
        return "Just say something!";
    }

    let qst = trim_text.ends_with("?");
    let upper = text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

    match (upper, qst) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}
