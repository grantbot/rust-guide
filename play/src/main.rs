use std::io;

fn main() {
    println!("Type something!");

    let input = io::stdin()
        .read_line()
        .ok()
        .expect("Failed to read line");

    println!("User input is {}", input);
}
