use colored::Colorize;
use rand::Rng;
use std::io::prelude::*;
use std::{cmp::Ordering, io};

fn main() {
    println!(
        "{}",
        "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\nWelcome to the Number Guessing Game!\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~".purple()
    );
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        print!("Enter your guess below.\n~> ");
        let mut guess = String::new();
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number.");
                continue;
            }
        };
        println!("You guessed -> {}...", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!(
                    "{0} {1}",
                    "Correct, you win!\nThe answer was in fact ->".green(),
                    secret_number
                );
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),

            Ordering::Less => println!("{}", "Too small.".red()),
        }
    }
}
