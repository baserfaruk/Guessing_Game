use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    println!("Guess the number! 1-30");

    let secret_number = rand::rng().random_range(1..=30);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

         io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
    }

    println!("Going to wait...");
    io::stdin().read_line(&mut String::new()).unwrap();
    
}
