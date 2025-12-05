//Bringing the IO library in scope
use std::io; 
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // function -> thread_rng and method -> gen_range
    let secret_number = rand::thread_rng().gen_range(1..=100); // format gen_range(start..=end) both inclusive
    println!("Your secret key is {secret_number}");

    println!("Please input your guess.");

    // ::new() creates a new type of the assigned (example, string below)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line"); //error handling

    println!("You guessed: {guess}");
}
