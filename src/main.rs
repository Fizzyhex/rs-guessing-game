use std::io;
use std::io::prelude::*;
use rand::Rng;
use std::cmp::Ordering;

fn pause(message: &str) {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    // Using write! allows us to directly write to the output and stop the cursor from going onto a newline
    write!(stdout, "{}", message).unwrap();
    // Flush the output to ensure the message is displayed immediately
    stdout.flush().unwrap();
    // Read a single byte - and then discard as we don't need it.
    // This will require the user to type something before we continue
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let stdin = io::stdin();
    let secret_number = rand::thread_rng().gen_range(1..=100); 
    println!("Guess the number! (0-100)");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
    
        stdin
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number!");
                continue;
            },
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("")
    }

    pause("Press enter to exit...");
}
