use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Generate a random number.
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("Guess the number!");
    println!("Secret number: {}", secret_number);
    println!("Please input your guess.");

    // Capture the input.
    // String is of the owned type and always UTF-8.
    // new is an associated function.
    let mut guess = String::new();

    // Read the input and saves as string to variable guess.
    // Bring in libary to read the line and handle Ok or Error.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Need to convert guess number into a number.
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please type a number.");

    // Print the input.
    println!("You guessed: {}", guess);

    // Compare the secret number and guess.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low."),
        Ordering::Equal => println!("You win."),
        Ordering::Greater => println!("Too high."),
    };
}
