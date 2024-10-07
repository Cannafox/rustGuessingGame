use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game");

    let random_number: u32 = rand::thread_rng().gen_range(1..=10);

    println!(
        "I am thinking about number {}, about what number I am thinking?",
        random_number
    );

    loop {
        println!("Enter your number: ");

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to get a number!");

        // Conversion, user_guess is mutable, so you can assign it with different type
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_guess.cmp(&random_number) {
            Ordering::Less => println!("Your number is smaller!"),
            Ordering::Greater => println!("Your number is bigger!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
