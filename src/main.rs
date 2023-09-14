use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing Game");
    println!("Please input your number");

    let mut guess = String::new();
    let secret_num = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Cannot parse string, please input a number");

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too low!"),
        Ordering::Equal => println!("You guessed right!"),
        Ordering::Greater => println!("Too high!")
    }

    println!("Your guess: {guess}");
}