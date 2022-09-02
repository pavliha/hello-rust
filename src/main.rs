use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Guess the number!");
        io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line");

        let guess: u32 = guess.trim().parse().expect("Please enter number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("It's too small"),
            Ordering::Equal => {
                println!("Bulls eye!");
                break;
            }
            Ordering::Greater => println!("It's too large"),
        }
    }
}
