use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Guessing the number!");

    loop {
        let mut guess: String = String::new();
        println!("Please input your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // println!("The secret number is {secret_number}");
        // println!("You guessed: {guess}");

        let guess: i32 = guess.trim().parse().expect("Invalid input");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too Low!"),
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
        }
    }
}
