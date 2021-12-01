use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Lucky 7 game!");

    
    let mut counter = 0;
    
    loop {
        let secret_number = rand::thread_rng().gen_range(1..8);
        if counter == 3 {
            break;
        }
        println!("Please input your guess between 1 and 7.");
            
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        println!("Secret Number was: {}", secret_number);
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulation, You win!");
                break;
            }
        }
        counter += 1;
    }
}
