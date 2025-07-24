use std::io;

fn main() {
    let mut count = 0;
    loop {
        let mut input = String::new();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        count += 1;
        if input.trim() == "The letter e" {
            println!("Number of trials: {}", count);
            return
        }
    }
}
