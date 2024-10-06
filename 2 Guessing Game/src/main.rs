use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number: {secret_number}.");

    // Infinite loop
    loop {
        println!("Please input your guess.");

        // Random number
        // mut makes a variable mutable
        // By default, variables are immutable
        let mut guess: String = String::new();

        // Reding the userinput to the mutable guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");

        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables.
        // parse() returns an Ok or Err value, that we are matching and handling the user made error if f.e. no number is given.
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            // The _ ist a "catchall" that catches every error type
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // println!("other alternative to print the guess is so: {guess}.");

        // Basically a switch case
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed to low."),
            Ordering::Greater => println!("You guessed to high."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
