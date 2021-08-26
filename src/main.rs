use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Capture the input.
    // String is of the owned type and always UTF-8.
    // new is an associated function.
    let mut guess = String::new();

    // Read the input.
    // Bring in libary to read the line and handle Ok or Error.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Print the input.
    println!("You guessed: {}", guess);
}
