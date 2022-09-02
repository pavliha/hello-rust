use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        print!("Guess the number: \n");

        io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line");

        println!("=========");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("It's too small"),
            Ordering::Equal => {
                println!("Bulls eye!");
                break;
            }
            Ordering::Greater => println!("It's too large"),
        
        }

        println!("=========");
    
    }
}
