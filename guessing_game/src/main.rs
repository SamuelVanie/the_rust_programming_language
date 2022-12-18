use rand::Rng;
use std::cmp::Ordering;
use std::io; // because io functions aren't in the prelude we have to bring them

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..=100); // immutable value (default)

    loop {
        println!("Please input your guess."); // println! is a macro use to print on stdin

        let mut guess = String::new(); // mutable value

        io::stdin()
            .read_line(&mut guess) // where to use the user input. & mean it's passing by reference
            .expect("Failed to read line"); // handling potential error

        let guess: u32 = match guess.trim().parse() { // match return a Result type which is either Ok or Err
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    
    }
}
