use std::io;

fn main() {
    println!("Guessing Game");
    println!("Please input your number");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess: {guess}");
}