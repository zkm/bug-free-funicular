use rand::Rng;
use std::error::Error;
use std::io::{self, Write};

pub fn run(_args: &[String]) -> Result<(), Box<dyn Error>> {
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Enter your guess: ");
        io::stdout().flush()?;

        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let guess: i32 = match buf.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid integer.");
                continue;
            }
        };

        if guess < secret {
            println!("Too low!");
        } else if guess > secret {
            println!("Too high!");
        } else {
            println!("You got it! The number was {}.", secret);
            break;
        }
    }
    Ok(())
}
