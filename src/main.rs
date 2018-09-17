use std::io;

fn main() {

    let secret_number : u32 = 45;
    
    println!("Guess the number!");

    loop{
    
        let mut guess = String::new();

        println!("Please input your guess: ");
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        
        if guess == secret_number {
            println!("You guessed the number!! Congrats");
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