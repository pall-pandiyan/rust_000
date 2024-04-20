use rand::Rng;

fn main() {
    println!("Guessing the number!");
    let secret_number = rand::thread_rng()
        .gen_range(1..=10);

    println!("Please input your guess: ");
    
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("The secret number is {secret_number}");
    println!("You guessed: {guess}");
}
