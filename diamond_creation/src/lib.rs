pub fn get_diamond(c: char) -> Vec<String> {
    let max_index = (c.to_ascii_uppercase() as u8 - b'A') as usize;
    let mut diamond = Vec::new();
    
    for i in 0..=max_index {
        let current_char = (b'A' + i as u8) as char;
        let outer_padding = " ".repeat(max_index - i);

        let line = if i == 0 {
            format!("{}{}{}", outer_padding, current_char, outer_padding)
        } else {
            let inner_spacing = " ".repeat(2 * i - 1);
            format!("{}{}{}{}{}", outer_padding, current_char, inner_spacing, current_char, outer_padding)
        };

        diamond.push(line);
    }
    
    for i in (0..max_index).rev() {
        diamond.push(diamond[i].clone());
    }

    diamond
}
