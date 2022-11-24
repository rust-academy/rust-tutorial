use std::cmp::Ordering;
use std::io;

use colored::*;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Guess the number");
        let mut read_guess = String::new();

        io::stdin()
            .read_line(&mut read_guess)
            .expect("Error reading from command line");

        let guess: u32 = match read_guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("{}", "Too small!".red()) }
            Ordering::Greater => { println!("{}", "Too big!".red()) }
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
