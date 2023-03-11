use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("---- Guess the number ----");
    println!("Please enter the maximum number to generate");

    let mut maximum = String::new();

    io::stdin()
        .read_line(&mut maximum)
        .expect("An error occured while reading the input");

    let maximum: u32 = maximum.trim().parse().expect("The given input is not a valid number");

    println!("Generating a number between 1 and {maximum}");
    let secret_number = rand::thread_rng().gen_range(1..=maximum);

    println!("The secret number is {secret_number}");
    println!("------------------------------------");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small"),
            Ordering::Greater => println!("Your guess was too high"),
            Ordering::Equal => { 
                println!("You win"); 
                break; 
            },
        }
        println!("---------------------");
    }
}
