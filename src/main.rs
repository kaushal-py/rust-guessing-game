use std::io;
extern crate rand;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("Guess the number!");

    loop{
    
        let mut guess = String::new();

        println!("Please input your guess: ");
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        
        if guess == secret_number {
            println!("You guessed the number!! The number was {}!", secret_number);
            break;
        }
        else if guess > secret_number{
            println!("Guess a lower number.");
        }
        else if guess < secret_number{
            println!("Guess a higher number");
        }

    }
}