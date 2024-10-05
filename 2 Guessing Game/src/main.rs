use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number :{secret_number}.");

    println!("Please input your guess.");

    // Random number
    // mut makes a variable mutable
    // By default, variables are immutable
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line...");

    println!("You guessed: {}", guess);

    // println!("other alternative to print the guess is so: {guess}.");
}
