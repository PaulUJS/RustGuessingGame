use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number: ");
    
    let random_num = rand::thread_rng().gen_range(1..=5);
    
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        println!("You guessed {guess}");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
