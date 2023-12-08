// First simple project in Rust

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the numbe!");
    // Generamos número secreto
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Pedimos al usuario su número
    loop {
        println!("Please input a guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {guess}");
        // Comparamos el número del usuario con el número secreto
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");
                break;
            }
        }
    }
}