// First simple project in Rust
use std::io;

fn main() {
    println!("Guess the numbe!");
    println!("Please input a guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}