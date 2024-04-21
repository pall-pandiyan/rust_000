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

        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

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
