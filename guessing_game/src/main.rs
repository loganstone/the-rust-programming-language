use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod common;

use crate::common::Guess;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let secret_number = Guess::new(secret_number);

    println!("The secret number is: {}", secret_number.value());

    loop {
        println!("Please enter the your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number.value()) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
