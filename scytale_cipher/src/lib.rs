pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.is_empty() || message.len() == num_columns as usize {
        return message.to_string();
    }

    let columns = num_columns as usize;
    let rows = (message.len() + columns - 1) / columns;
    
    let mut grid = vec![vec![' '; columns]; rows];
    for (i, ch) in message.chars().enumerate() {
        let row = i / columns;
        let col = i % columns;
        grid[row][col] = ch;
    }

    let mut result = String::new();
    for col in 0..columns {
        for row in 0..rows {
            result.push(grid[row][col]);
        }
    }

    result.trim_end().to_string()
}
