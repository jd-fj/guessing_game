use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..3);

    // next line used for testing purposes only
    // println!("The secret number is: {}", secret_number);

    loop {        
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        // shadow the previous value of guess. Shadowing lets us reuse the guess variable name. The shadow is on the left, the original is on the right. we use trim because read_line requires enter to be pressed and adds a "\n" to the end of the string.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You've guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, sucker"),
            Ordering::Greater => println!("Too big, ya muck!"),
            Ordering::Equal => {
                println!("You win! No more mean names!");
                break;
            }
        }
    }
}